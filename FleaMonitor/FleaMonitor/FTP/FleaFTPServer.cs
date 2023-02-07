using FubarDev.FtpServer;
using FubarDev.FtpServer.AccountManagement.Anonymous;
using FubarDev.FtpServer.FileSystem.DotNet;
using Microsoft.Extensions.DependencyInjection;
using System.IO;
using System.Threading;
using System.Threading.Tasks;

namespace FleaMonitor.FTP
{
    public class FleaFTPServer
    {
        private static readonly string FLEA_FTP_FOLDER = "FleaFTP";
        private static readonly string FLEA_FOLDER = "Files";

        private readonly ServiceCollection _services = new();
        private ServiceProvider? _serviceProvider;
        private IFtpServerHost? _ftpServerHost;

        public bool IsStarted { get; private set; } = false;

        public async Task Init()
        {
            var ftpPath = Path.Combine(Path.GetTempPath(), FLEA_FTP_FOLDER);
            
            // use %TEMP%/TestFtpServer as root folder
            _services.Configure<DotNetFileSystemOptions>(opt => opt
               .RootPath = ftpPath);

            Directory.CreateDirectory(Path.Combine(ftpPath, FLEA_FOLDER));
            
            // Add FTP server services
            // DotNetFileSystemProvider = Use the .NET file system functionality
            // AnonymousMembershipProvider = allow only anonymous logins
            _services.AddFtpServer(builder => builder
               .UseDotNetFileSystem() // Use the .NET file system functionality
               .EnableAnonymousAuthentication()); // allow anonymous logins

            // Configure the FTP server
            _services.Configure<FtpServerOptions>(opt => opt.ServerAddress = "*");
            _services.AddSingleton<IAnonymousPasswordValidator>(new NoValidation());
            
            // Build the service provider
            _serviceProvider = _services.BuildServiceProvider(true);
            
            // Initialize the FTP server
            _ftpServerHost = _serviceProvider.GetRequiredService<IFtpServerHost>();

            // Start the FTP server
            await _ftpServerHost.StartAsync(CancellationToken.None).ConfigureAwait(false);

            IsStarted = true;
        }        

        public async Task Stop()
        {
            if(_ftpServerHost is not null)
            {
                await _ftpServerHost.StopAsync(CancellationToken.None).ConfigureAwait(false);
                _ftpServerHost = null;
            }

            if (_serviceProvider is not null)
            {
                _serviceProvider.Dispose();
                _serviceProvider = null;
            }

            IsStarted = false;
        }
    }
}
