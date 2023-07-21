export const convertNumberToLetter = (value: number): string => {
  return (value + 9).toString(36).toUpperCase();
};
