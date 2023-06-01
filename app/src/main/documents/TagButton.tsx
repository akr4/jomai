import { Button } from '../../components/Button';
import { Tag } from '../../components/Tag';
import React from 'react';

export const TagButton: React.FC<{
  tag: string;
  onTagClick: () => void;
}> = React.memo(({ tag, onTagClick }) => {
  return (
    <Button key={tag} variant="link" onClick={onTagClick}>
      <Tag>{tag}</Tag>
    </Button>
  );
});
TagButton.displayName = 'TagButton';
