using System.Linq;

namespace DungeonCrawler.Networking.Datagrams
{
    public class Ack : Datagram
    {
        public ulong AckIndex { get; set; }
        public Ack(string datagram) =>
            AckIndex = ulong.Parse(datagram);

        public static string CreateString(ulong ackIndex) => $"ACK::{ackIndex}";
    }
}