import styled, { keyframes } from 'styled-components';

export const AlertBlock = styled.div`
  position: fixed;
  bottom: 0;
  left: 50%;
  transform: translateX(-50%);

  z-index: 100;
`;

export const AlertWrapper = styled.div<{
  type_of_message: string;
}>`
  position: relative;
  display: flex;
  flex-flow: row wrap;
  justify-content: space-between;
  align-items: center;

  padding: 0.75rem 1rem 0.75rem 2.875rem;
  margin-bottom: 1rem;

  border-radius: 0.5rem;
  background-color: var(--color-black);
  font-size: 0.875rem;
  font-weight: 400;

  animation:
    ${() => shakeAnimation()} 0.5s ease-out forwards,
    ${() => outAnimation()} 0.2s ease-in 5s forwards;

  p {
    margin-right: 1.5rem;
    line-height: 1.5;
    color: ${(props) => {
      if (props.type_of_message === 'ERROR') {
        return `var(--color-red)`;
      } else if (props.type_of_message === 'SUCCESS') {
        return 'var(--color-green)';
      } else {
        return 'var(--color-white)';
      }
    }};
  }
`;

const shakeAnimation = () => keyframes`
    0% {
      transform: translateY(calc(100% + 1rem));
    }
    80% {
      transform: translateY(calc(100% - 50px - 0.3rem));
    }
    90% {
      transform: translateY(calc(100% - 50px + 0.3rem));
    }
    100% {
      transform: translateY(calc(100% - 50px));
    }
`;

const outAnimation = () => keyframes`
    0% {
      transform: translateY(calc(100% - 50px));
    }
    100% {
      transform: translateY(calc(100% + 1rem));
    }
`;
