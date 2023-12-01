/**
 * Function to parse the digits in each row
 * @param nums array containing the parsed numbers 
 * @returns the first and last digit 
 */
const parseDigits = (nums: number[] | []): number => {
  if (nums.length === 0) return 0;
  const first = nums[0];
  const last = nums[nums.length - 1];
  return parseInt(`${first}${last}`);
};

export const part1 = (input: string) => {
  let sum = 0;
  input.split("\n").forEach((line) => {
    const chars = line.split("");
    
    // this is the same as:
    // const nums = chars.map((char) => parseInt(char)).filter(num=>!isNaN(num));
    const nums = chars.map(Number).filter(Boolean);
    
    sum += parseDigits(nums);
  });
  return sum;
};

export const part2 = (input: string) => {
  return input;
};