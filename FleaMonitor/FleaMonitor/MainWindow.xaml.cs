using FleaMonitor.Auxiliary;
using FleaMonitor.Model;
using FleaMonitor.Server;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;

namespace FleaMonitor
{    
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        private static readonly string FLEA_SERVER_IP = "192.168.0.22";
        private static readonly int FLEA_SERVER_PORT = 1972;
        private static readonly string FLEA_MONITOR_VERSION = "Flea Monitor v0.1 ready.\n\n";

        private FleaInfo _fleaInfo = new();
        private bool _commandInExecution = false;

        public MainWindow()
        {
            InitializeComponent();
            InitData();

            _fleaInfo.Txt = FLEA_MONITOR_VERSION;
        }

        private void InitData()
        {
            dirListView.ItemsSource = new List<FleaDirectory>();
            infoTextBlock.DataContext = _fleaInfo;

            commandComboBox.ItemsSource = new List<string>
            {
                "version",
                "bash",
                "ftpscreenshot",
                "screenshot",
                "log",
                "proclist",
                "kill",
                "upload",
                "dir",
                "cd",
                "getfile",
                "quit"
            };
        }

        private async Task<byte[]> SendCommand(string cmd, string param, FleaInfo? fleaInfo)
        {
            _commandInExecution = true;

            try
            {
                return await FleaServer.SendCommand(CommandProcessor.CreateXML(cmd, param), FLEA_SERVER_IP, FLEA_SERVER_PORT, fleaInfo);
            }
            catch (ArgumentNullException ane)
            {
                _fleaInfo.Txt = $"ArgumentNullException : {ane}\n";
            }
            catch (System.Net.Sockets.SocketException se)
            {
                _fleaInfo.Txt = $"SocketException : {se}\n";
            }
            catch (Exception ex)
            {
                _fleaInfo.Txt = $"Unexpected exception : {ex}\n";
            }
            finally
            {
                _commandInExecution = false;
            }

            return Array.Empty<byte>();
        }

        private async void ReadDirectory()
        {
            var result = await SendCommand("dir", string.Empty, null);
            var str = Encoding.UTF8.GetString(result, 0, result.Length);
            var lines = str.Split(new char[] { '\n' });
            var lst = new List<FleaDirectory>();

            lst.AddRange(lines.Select(s => new FleaDirectory
            {
                Txt = s.Replace("\r", "")
            }));

            await Application.Current.Dispatcher.InvokeAsync(() =>
            {
                dirListView.ItemsSource = lst;
            });
        }

        private async void SendButton_Click(object sender, RoutedEventArgs e)
        {
            var idx = commandComboBox.SelectedIndex;
            
            if (idx == -1)
            {
                MessageBox.Show("Please select a command.");
                return;
            }

            if (_commandInExecution)
            {
                MessageBox.Show("Command is executed now..");
                return;
            }

            var cmd = commandComboBox.SelectedItem.ToString();
            var result = await SendCommand(cmd!, string.Empty, _fleaInfo);
        }

        private async void Window_Loaded(object sender, RoutedEventArgs e) => await Task.Run(ReadDirectory);
    }
}
