using System.Net;

namespace DungeonCrawler.Networking
{
    /// <summary>
    /// Represents the information for any datagram sent which
    /// requires a confirmation on the other side before
    /// resolution. Used in the NetworkDatagramHandler to
    /// resend any reliable datagram which has timed out.
    /// </summary>
    public class AckResolver
    {
        public ulong AckIndex { get; set; }
        public long TicksStart { get; set; }
        public string Message { get; set; }
    }
}