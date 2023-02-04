namespace FleaMonitor.Auxiliary
{
    public class CommandProcessor
    {
        public static string CreateXML(string command, string value)
        {
            return $"<Command name='{command}' value='{value}'></Command>";
        }
    }
}
