using System.Windows;

namespace FleaMonitor.Dialog
{
    /// <summary>
    /// Interaction logic for SettingsWindow.xaml
    /// </summary>
    public partial class SettingsWindow : Window
    {
        public SettingsWindow()
        {
            InitializeComponent();
        }

        private void Button_Click(object sender, RoutedEventArgs e)
        {
            if (fleaIPTextBlox.Text.Length == 0)
            {
                MessageBox.Show("Please enter IP address", "Error", MessageBoxButton.OK, MessageBoxImage.Error);
                return;
            }

            if (encryptionKeyTextBlox.Text.Length is > 0 and not MainWindow.ENCYPTION_KEY_LENGTH)
            {
                MessageBox.Show("Ecryption key must be 32 bytes long", "Error", MessageBoxButton.OK, MessageBoxImage.Error);
                return;
            }

            DialogResult = true;
        }
    }
}
