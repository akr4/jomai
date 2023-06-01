import { Row } from '../../components/Row';
import { TagButton } from './TagButton';
import React from 'react';

export const TagRow: React.FC<{
  tags: string[];
  onTagClick: (tag: string) => void;
}> = React.memo(({ tags, onTagClick }) => {
  return (
    <Row gap="small">
      {tags.map((tag) => {
        return (
          <TagButton key={tag} tag={tag} onTagClick={() => onTagClick(tag)} />
        );
      })}
    </Row>
  );
});
TagRow.displayName = 'TagRow';
