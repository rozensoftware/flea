using System;
using System.Collections.Generic;
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
using System.Windows.Shapes;

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
