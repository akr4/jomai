import React, { forwardRef, RefObject } from 'react';
import { Icon } from '../../components/Icon';
import { Row } from '../../components/Row';
import { Spacer } from '../../components/Spacer';
import { TextField } from '../../components/TextField';
import { Column } from '../../components/Column';
import { Tag } from '../../components/Tag';

type Props = {
  query: string;
  tags: string[];
  onInputFocus: () => void;
  onQueryChange: (query: string) => void;
  onTagRemove: (tag: string) => void;
  inputRef?: RefObject<HTMLInputElement>;
};

export const SearchForm = forwardRef<HTMLDivElement, Props>(
  (
    { query, tags, inputRef, onInputFocus, onQueryChange, onTagRemove },
    ref,
  ) => {
    return (
      <Column ref={ref} padding="medium">
        <Row align="center" padding="medium">
          <Icon iconName="magnifying-glass" textStyle="body1" />
          <Spacer width="small" />
          <TextField
            ref={inputRef}
            value={query}
            type="search"
            onChange={(e) => onQueryChange(e.target.value)}
            onFocus={onInputFocus}
            width="medium"
          />
        </Row>
        <Row gap="small">
          {tags.map((tag) => {
            return (
              <Tag key={tag} onClose={() => onTagRemove(tag)}>
                {tag}
              </Tag>
            );
          })}
        </Row>
      </Column>
    );
  },
);
SearchForm.displayName = 'SearchForm';
