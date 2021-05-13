using System.Linq;

namespace DungeonCrawler.Networking.Datagrams
{
    public class Ping : Datagram
    {
        public static string CreateString() => $"PNG";
    }
}