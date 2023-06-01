import React from 'react';

export const useHover = () => {
  const [isHovered, setIsHovered] = React.useState(false);

  const handleMouseEnter = () => {
    setIsHovered(true);
  };

  const handleMouseLeave = () => {
    setIsHovered(false);
  };

  return {
    isHovered,
    handleMouseEnter,
    handleMouseLeave,
  };
};
