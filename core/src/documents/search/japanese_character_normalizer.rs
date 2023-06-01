use tantivy::tokenizer::{BoxTokenStream, Token, TokenFilter, TokenStream};

#[derive(Clone)]
pub struct JapaneseCharacterTypeNormalizer;

impl TokenFilter for JapaneseCharacterTypeNormalizer {
    fn transform<'a>(&self, token_stream: BoxTokenStream<'a>) -> BoxTokenStream<'a> {
        BoxTokenStream::from(JapaneseCharacterTypeNormalizerTokenStream { tail: token_stream })
    }
}

struct JapaneseCharacterTypeNormalizerTokenStream<'a> {
    tail: BoxTokenStream<'a>,
}

impl<'a> TokenStream for JapaneseCharacterTypeNormalizerTokenStream<'a> {
    fn advance(&mut self) -> bool {
        if !self.tail.advance() {
            return false;
        }

        // カタカナをひらがなに変換する
        // ただしカタカナ語のひらがなでの検索はうまくいかないことが多い。
        // ひらがなでのクエリはトークナイズがカタカナ語と異なる結果となるため。「めも」-> 「め」「も」など
        let mut text = kana::wide2ascii(&self.token().text);
        text = kana::half2kana(&text);
        text = kana::kata2hira(&text);

        self.tail.token_mut().text = text;
        true
    }

    fn token(&self) -> &Token {
        self.tail.token()
    }

    fn token_mut(&mut self) -> &mut Token {
        self.tail.token_mut()
    }
}
