using FleaMonitor.Model;
using System;
using System.Linq;
using System.Net;
using System.Text;
using System.Threading.Tasks;

namespace FleaMonitor.Server
{
    public class FleaServer
    {
        private static readonly int MAX_BUFFER = 1024;

        /// <summary>
        /// Maximum returned data to log in verbose mode. If the data is longer than this value, it will be truncated.
        /// </summary>
        private static readonly int MAX_VERBOSE_BUFFER = 1024 * 4;
        
        /// <summary>
        /// Sends command in the form of XML as array of bytes to the remote server over tcp connection. It uses sockets.
        /// </summary>
        /// <param name="cmd">XML string that can be read by Flea Server</param>
        /// <param name="ip">Flea Server IP</param>
        /// <param name="port">Flea Server port</param>
        /// <param name="fleaInfo">FleaInfo object that will be updated with the response from the server</param>
        /// <returns>Response from the server as byte array</returns>
        /// <exception cref="Exception"></exception>
        /// <exception cref="System.Net.Sockets.SocketException"></exception>
        /// <exception cref="ArgumentNullException"></exception>
        public async static Task<byte[]> SendCommand(string cmd, string ip, int port, FleaInfo? fleaInfo, bool showAllResult = false)
        {
            byte[] returnBuffer = { };
            int readBytesNumber = 0;
            
            // Connect to a remote device.
            // Establish the remote endpoint for the socket.
            // This example uses port 11000 on the local computer.
            IPAddress ipAddress = IPAddress.Parse(ip);
            IPEndPoint remoteEP = new(ipAddress, port);

            System.Net.Sockets.Socket? sender = null;
            
            try
            {
                // Create a TCP/IP  socket.
                sender = new System.Net.Sockets.Socket(ipAddress.AddressFamily,
                    System.Net.Sockets.SocketType.Stream, System.Net.Sockets.ProtocolType.Tcp);

                // Connect the socket to the remote endpoint. Will throw any errors.
                await sender.ConnectAsync(remoteEP);

                if (fleaInfo is not null)
                {
                    fleaInfo.Txt = $"Connected to {sender.RemoteEndPoint?.ToString()}\n";
                    fleaInfo.Txt = $"Command XML: {cmd}\n";
                }

                // Encode the data string into a byte array.
                byte[] msg = Encoding.UTF8.GetBytes(cmd);

                // Send the data through the socket.
                int bytesSent = await sender.SendAsync(msg);

                byte[] buffer = new byte[MAX_BUFFER];
                int bytesRec = 0;

                do
                {
                    // Receive the response from the remote device.
                    bytesRec = await sender.ReceiveAsync(buffer);
                    if (bytesRec > 0)
                    {
                        readBytesNumber += bytesRec;
                        returnBuffer = returnBuffer.Concat(buffer[0..bytesRec]).ToArray();
                    }
                }
                while (bytesRec > 0);

                if (fleaInfo is not null)
                {
                    fleaInfo.Txt = $"Read {readBytesNumber} bytes from Flea Server:\n";
                    if (readBytesNumber > MAX_VERBOSE_BUFFER && !showAllResult)
                    {
                        fleaInfo.Txt = string.Concat(Encoding.UTF8.GetString(returnBuffer, 0, MAX_VERBOSE_BUFFER), "...\n");
                    }
                    else
                    {
                        fleaInfo.Txt = string.Concat(Encoding.UTF8.GetString(returnBuffer, 0, readBytesNumber), "\n");
                    }
                }

                return returnBuffer;
            }
            finally
            {
                // Release the socket.
                sender?.Shutdown(System.Net.Sockets.SocketShutdown.Both);
                sender?.Close();
                if (fleaInfo is not null)
                {
                    fleaInfo.Txt = "Connection closed.\n";
                }
            }
        }
    }
}
