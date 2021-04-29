namespace DungeonCrawler.Networking.Datagrams
{
    /// <summary>
    /// A datagram which informs the recipient to
    /// resend it's list of reliable datagram packets
    /// which have not been recieved by the sender yet.
    /// </summary>
    public class Resend : Datagram
    {
        public static string CreateString() => "RES";
    }
}