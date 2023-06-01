import { getContainingFolder } from '../../api/core';
import { open } from '@tauri-apps/api/shell';
import { writeText } from '@tauri-apps/api/clipboard';

export const openContainingFolder = async (path: string) => {
  const folder = await getContainingFolder(path);
  await open(encodeURI(`file://${folder}`));
};

export const openFile = async (path: string) => {
  await open(encodeURI(`file://${path}`));
};

export const copyToClipboard = async (text: string) => {
  await writeText(text);
};
