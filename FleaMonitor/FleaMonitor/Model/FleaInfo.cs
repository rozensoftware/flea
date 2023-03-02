using System.ComponentModel;

namespace FleaMonitor.Model
{
    public class FleaInfo : INotifyPropertyChanged
    {
        private string? _txt;
        
        public string? Txt
        {
            get { return _txt; }
            set
            {
                if(value is not null)
                {
                    _txt += value;
                    OnPropertyChanged(nameof(Txt));
                }
            }
        }
        public void ClearTxt()
        {
            _txt = string.Empty;
            OnPropertyChanged(nameof(Txt));
        }
        
        public event PropertyChangedEventHandler? PropertyChanged;
        
        private void OnPropertyChanged(string propertyName)
        {
            PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
        }
    }
}
