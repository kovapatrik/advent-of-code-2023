# -*- coding: utf-8 -*-
"""
Spyder Editor

This is a temporary script file.
"""

import numpy as np
# import itertools
import time

def str_diff_1(str1,str2):
    return sum(p1!=p2 for p1,p2 in zip(str1,str2))==1

def check_pattern_p2(pattern):
    used_mod = 0
    pattern_temp = pattern.split('\n')
    rr = range(0,len(pattern_temp))
    cr = range(0, len(pattern_temp[0]))
    print('maxr',rr.stop-1,'maxc',cr.stop-1)
    #row search
    for i,line in enumerate(pattern_temp[:-1]):
        # print(line, pattern_temp[i+1])
        if ((line == pattern_temp[i+1]) | ((used_mod ==0) & str_diff_1(line,pattern_temp[i+1]))) :
            reflection1 = i
            reflection2 = i+1
            while((rr.count(reflection1)>0) & (rr.count(reflection2)>0)) & ((pattern_temp[reflection1]==pattern_temp[reflection2]) | ((used_mod ==0) & str_diff_1(pattern_temp[reflection1],pattern_temp[reflection2]))):
                if ((used_mod ==0) & str_diff_1(pattern_temp[reflection1],pattern_temp[reflection2])):
                    used_mod = 1
                print('h\t',reflection1,'\t',pattern_temp[reflection1],used_mod)
                print('h\t',reflection2,'\t',pattern_temp[reflection2],used_mod)
                reflection1 -=1
                reflection2 +=1

                if (((rr.count(reflection1)>0) & (rr.count(reflection2)>0))==0):
                    if used_mod:
                        return (i+1)*100
                    else:
                        break
            used_mod=0
    used_mod = 0
    
    pattern_temp2 = [''.join(x) for x in zip(*pattern_temp)]
    for i,line in enumerate(pattern_temp2[:-1]):
        # print(line, pattern_temp2[i+1])
        if ((line == pattern_temp2[i+1]) | ((used_mod ==0) & str_diff_1(line,pattern_temp2[i+1]))) :
            reflection1 = i
            reflection2 = i+1
            while((cr.count(reflection1)>0) & (cr.count(reflection2)>0)) & ((pattern_temp2[reflection1]==pattern_temp2[reflection2]) | ((used_mod ==0) & str_diff_1(pattern_temp2[reflection1],pattern_temp2[reflection2]))):
                if ((used_mod ==0) & str_diff_1(pattern_temp2[reflection1],pattern_temp2[reflection2])):
                    used_mod = 1
                print('v\t',reflection1,'\t',pattern_temp2[reflection1],used_mod)
                print('v\t',reflection2,'\t',pattern_temp2[reflection2],used_mod)
                reflection1 -=1
                reflection2 +=1

                if (((cr.count(reflection1)>0) & (cr.count(reflection2)>0))==0):
                    if used_mod:
                        return (i+1)
                    else:
                        break
            used_mod=0
    
    # for c in np.arange(len(pattern_temp[0])-1):
    #     colstr = ''.join([l[c] for l in pattern_temp])
    #     colstrnxt = ''.join([l[c+1] for l in pattern_temp])
    #     if ((colstr == colstrnxt )| ((used_mod ==0) & str_diff_1(colstr,colstrnxt))):
    #         reflection1 = c
    #         reflection2 = c+1
    #         str1 = ''.join([l[reflection1] for l in pattern_temp])
    #         str2 = ''.join([l[reflection2] for l in pattern_temp])
    #         while(cr.count(reflection1)>0) & (cr.count(reflection2)>0) & (((str1==str2))| ((used_mod ==0) & str_diff_1(str1,str2))):
    #             if ((used_mod ==0) & str_diff_1(str1,str2)):
    #                     used_mod =1
    #             print('v\t',reflection1,'\t',str1,used_mod)
    #             print('v\t',reflection2,'\t',str2,used_mod)
    #             reflection1 -=1
    #             reflection2 +=1
    #             if (((cr.count(reflection1)>0) & (cr.count(reflection2)>0))==0) & (used_mod):
    #                 return (c+1)
    #             else:
    #                 str1 = ''.join([l[reflection1] for l in pattern_temp])
    #                 str2 = ''.join([l[reflection2] for l in pattern_temp])
    #         used_mod = 0
            
            
start = time.time()
content = open('in.txt').read().split('\n\n')
val = 0
for ip,pattern in enumerate(content):
    print(ip)
    val += check_pattern_p2(pattern)


print('Valid records = %d after %f s' %(val,time.time()-start))
