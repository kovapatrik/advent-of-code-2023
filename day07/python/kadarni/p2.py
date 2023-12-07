import numpy as np
import re
from collections import defaultdict

content = open('/home/istkad01/voyager/.venv/day07/in.txt').readlines()

rank_dict = {
    (5,5,5,5,5) : 7,
    (1,4,4,4,4) : 6,
    (2,2,3,3,3) : 5,
    (1,1,3,3,3) : 4,
    (1,2,2,2,2) : 3,
    (1,1,1,2,2) : 2,
    (1,1,1,1,1) : 1
}

card_vals = {
    'A':13,'K':12,'Q':11,'T':10,'9':9,'8':8,'7':7,'6':6,'5':5,'4':4,'3':3,'2':2,'J':1
}



def similars(cards):
    return rank_dict[tuple(np.sort(np.asarray([len(re.findall(c, cards)) for c in cards])))]


ranks = list()


for i,line in enumerate(content):
    ranks.append( {
        'cards':line.split()[0],
        'bid':float(line.split()[1]),
        'strength':similars(line.split()[0]),
        'rank':0,
        'value':float(card_vals[line.split()[0][0]]*14**4+ card_vals[line.split()[0][1]]*14**3 + card_vals[line.split()[0][2]]*14*14 + card_vals[line.split()[0][3]]*14 + card_vals[line.split()[0][4]])
        })
    if 'J' in ranks[i]['cards']:
        if ranks[i]['strength'] in [6,5]:
            ranks[i]['strength']=7
        elif ranks[i]['strength']==4:
            ranks[i]['strength']=6
        elif ranks[i]['strength']==3:
            if sum('J' in c for c in ranks[i]['cards'])==1:
                ranks[i]['strength']=5
            elif sum('J' in c for c in ranks[i]['cards'])==2:
                ranks[i]['strength']=6
        elif ranks[i]['strength']==2:
            ranks[i]['strength']=4
        elif ranks[i]['strength']==1:
            ranks[i]['strength']=2
max_rank = len(content)
all_values = [a['value'] for a in ranks ]
for str in np.arange(7,0,-1):
    highest_rank = [a['strength']==str for a in ranks]
    if sum(highest_rank)>1:

        values = [a['value'] for a in ranks if a['strength']==str]
        sorted_values = np.sort(np.asarray(values))
        for i,sval in enumerate(sorted_values):
            # print(sval)
            ranks[all_values.index(sval)]['rank'] = max_rank-len(values)+1+i
        max_rank-=len(values)
    elif sum(highest_rank)==1:
        ranks[highest_rank.index(True)]['rank'] = max_rank
        max_rank-=1


print(np.sum(np.asarray([a['bid']*a['rank'] for a in ranks])))