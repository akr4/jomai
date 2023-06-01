import 'dayjs/locale/ja';
import LocalizedFormat from 'dayjs/plugin/localizedFormat';
import dayjs from 'dayjs';

dayjs.extend(LocalizedFormat);
dayjs.locale('ja');
