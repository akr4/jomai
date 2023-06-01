import { Resource } from 'i18next';

const en = {
  translation: {
    common: {
      websiteUrl: 'https://jomai.app/',
      documents_one: '{{count, number}} document',
      documents_other: '{{count, number}} documents',
      timestamp:
        '{{timestamp, datetime(dateStyle: medium; timeStyle: medium;)}}',
    },
    tab: {
      documents: 'Documents',
      watches: 'Paths',
    },
    license: {
      alert: {
        title: 'New version available',
        message:
          'A new version of the software is available. Please download and use the latest version.',
        button: 'Exit',
      },
    },
    pathRecommendations: {
      header: {
        title: 'Welcome to Jomai!',
        text1:
          'To begin, register the folders that contain the documents to make them searchable.',
        text2:
          'Check the folders you wish to register and press the Start button. You can also register later.',
      },
      paths: {
        documents: 'Documents folder (recommended)',
        obsidian: 'Obsidian Vault folder',
      },
      footer: {
        button: 'Start',
      },
    },
    documents: {
      found_zero: 'No documents found',
      found: 'Found $t(common.documents)',
      createdAt: 'Created: $t(common.timestamp)',
      modifiedAt: 'Updated: $t(common.timestamp)',
      openInDefaultApp: 'Open (↩)',
      menu: {
        openInDefaultApp: 'Open',
        openContainingFolder: 'Open containing folder',
        copyPathToClipboard: 'Copy path to clipboard',
      },
      sort: {
        relative: 'Related',
        timestamp: 'Newest',
      },
    },
    watches: {
      addPath: 'Add path',
      documentCount: '$t(common.documents)',
      createdAt: 'Added: $t(common.timestamp)',
      explanation:
        'Documents under the registered paths are monitored and made searchable. Newly created files are automatically added to the index.',
      menu: {
        delete: 'Delete',
      },
      job: {
        status: {
          active: 'Syncing',
          adding: 'Adding',
          deleting: 'Deleting',
        },
      },
      errors: {
        title: 'Error',
        'parent-child-relationship':
          'Parent-child relationship paths cannot be added',
        'watch-already-exists': 'Watch already exists',
        other: 'Something went wrong',
      },
    },
  },
};

const ja: RecursivePartial<typeof en> = {
  translation: {
    common: {
      websiteUrl: 'https://jomai.app/ja/',
      documents_one: '{{count, number}}',
      documents_other: '{{count, number}}',
      timestamp:
        '{{timestamp, datetime(dateStyle: medium; timeStyle: medium;)}}',
    },
    tab: {
      documents: 'ドキュメント',
      watches: 'パス',
    },
    license: {
      alert: {
        title: '最新版をご利用ください',
        message:
          '新しいバージョンのソフトウェアが利用可能です。ダウンロードしてご利用ください。',
        button: '終了',
      },
    },
    pathRecommendations: {
      header: {
        title: 'Jomai へようこそ！',
        text1:
          'はじめに、ドキュメントが含まれているフォルダーを登録して検索可能にします。',
        text2:
          '登録するフォルダーにチェックをいれて開始ボタンを押してください。後から登録することもできます。',
      },
      paths: {
        documents: 'Documents フォルダー (おすすめ)',
        obsidian: 'Obsidian Vault フォルダー',
      },
      footer: {
        button: '開始',
      },
    },
    documents: {
      found_zero: 'ドキュメントが見つかりませんでした',
      found: '$t(common.documents) 件見つかりました',
      createdAt: '作成日時: $t(common.timestamp)',
      modifiedAt: '更新日時: $t(common.timestamp)',
      openInDefaultApp: '開く (↩)',
      menu: {
        openInDefaultApp: '開く',
        openContainingFolder: 'フォルダを開く',
        copyPathToClipboard: 'パスをコピー',
      },
      sort: {
        relative: '関連度順',
        timestamp: '日付順',
      },
    },
    watches: {
      addPath: 'パスを追加',
      documentCount: '$t(common.documents) ドキュメント',
      createdAt: '登録日時: $t(common.timestamp)',
      explanation:
        '登録されたパス以下のドキュメントを監視して検索可能にします。新規作成されたファイルは自動的に検索対象になります。',
      menu: {
        delete: '削除',
      },
      job: {
        status: {
          active: '同期中',
          adding: '追加中',
          deleting: '削除中',
        },
      },
      errors: {
        title: 'エラー',
        'parent-child-relationship': '親子関係のパスは追加できません',
        'watch-already-exists': '既に追加されているパスです',
        other: 'エラーが発生しました',
      },
    },
  },
};

export const resources: Resource = {
  en,
  ja,
};

// https://stackoverflow.com/a/51365037/123673
type RecursivePartial<T> = {
  [P in keyof T]?: T[P] extends (infer U)[]
    ? RecursivePartial<U>[]
    : T[P] extends object
    ? RecursivePartial<T[P]>
    : T[P];
};
