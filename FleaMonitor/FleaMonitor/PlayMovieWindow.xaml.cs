using System;
using System.Windows;

namespace FleaMonitor
{
    /// <summary>
    /// Interaction logic for PlayMovieWindow.xaml
    /// </summary>
    public partial class PlayMovieWindow : Window
    {
        public PlayMovieWindow()
        {
            InitializeComponent();
        }

        private void mediaElement_MediaEnded(object sender, RoutedEventArgs e)
        {
            mediaElement.Position = TimeSpan.FromMilliseconds(1);
        }
    }
}
