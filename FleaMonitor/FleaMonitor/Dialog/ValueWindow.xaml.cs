using System.Windows;

namespace FleaMonitor.Dialog
{
    /// <summary>
    /// Interaction logic for ValueWindow.xaml
    /// </summary>
    public partial class ValueWindow : Window
    {
        public ValueWindow()
        {
            InitializeComponent();
        }

        private void Button_Click(object sender, RoutedEventArgs e)
        {
            if (valueTextBox.Text.Length == 0)
            {
                MessageBox.Show("Please enter a value.", "Error", MessageBoxButton.OK, MessageBoxImage.Error);
                return;
            }
            
            DialogResult = true;
        }
    }
}
