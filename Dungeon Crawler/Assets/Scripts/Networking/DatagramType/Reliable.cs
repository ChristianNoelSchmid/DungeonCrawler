using System.Linq;

namespace DungeonCrawler.Networking.Datagrams
{
    /// <summary>
    /// Represents a reliable datagram transmission, with an
    /// acknowledgement index, and the associated data.
    /// </summary>
    public class Reliable : Datagram
    {
        public ulong AckIndex { get; set; }
        public string Data { get; set; }

        public Reliable(string datagram)
        {
            var segs = datagram.Split(new string[] { "::" }, System.StringSplitOptions.None);
            AckIndex = ulong.Parse(segs[0]);
            Data = string.Join("::", segs.Skip(1));
        }

        public static string CreateString(ulong ackIndex, string data) => 
            $"REL::{ackIndex}::{data}";
    }
}