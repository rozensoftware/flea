using System.Collections.Generic;
using System.Text;
using System.Windows.Documents;

namespace FleaMonitor.Auxiliary
{
    public class Helper
    {
        public static string CleanLog(string? log)
        {
            if(string.IsNullOrEmpty(log))
            {
                return string.Empty;
            }

            var strLen = log.Length;
            var retStr = new StringBuilder(strLen);

            for(var i = 0; i < strLen; i++)
            {
                if (log[i] == '[')
                {
                    if(i + 2 < strLen && log[i + 2] == ']')
                    {
                        retStr.Append(log[i + 1]);
                        i += 2;
                        continue;
                    }
                }

                retStr.Append(log[i]);
            }

            var numericKeysList = new Dictionary<string, string>
            {
                { "[Key0]", "0" },
                { "[Key1]", "1" },
                { "[Key2]", "2" },
                { "[Key3]", "3" },
                { "[Key4]", "4" },
                { "[Key5]", "5" },
                { "[Key6]", "6" },
                { "[Key7]", "7" },
                { "[Key8]", "8" },
                { "[Key9]", "9" },
                { "[LeftBracket]", "[" },
                { "[RightBracket]", "]" },
                { "[Comma]", "," },
                { "[Dot]", "." },
                { "[Equal]", "=" },
                { "[Minus]", "-" },
                { "[Semicolon]", ";" },
                { "[Apostrophe]", "'" },
                { "[BackSlash]", "\\" },
                { "[Down]", "" },
                { "[Up]", "" },
                { "[Right]", "" },
                { "[Left]", "" },
                { "[Space]", " " },
                { "[Home]", "" },
                { "[Escape]", "" },
                { "[Slash]", "/" }
            };

            var str = retStr.ToString();
            
            foreach(var s in numericKeysList)
            {
                str = str.Replace(s.Key, s.Value);
            }

            return str;
        }
    }
}
