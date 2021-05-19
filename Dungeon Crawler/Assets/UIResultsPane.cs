using Assets.Scripts.Models;
using DungeonCrawler.Monobehaviours;
using System.Linq;
using UnityEngine;
using UnityEngine.UI;

public class UIResultsPane : UIGroup
{
    [SerializeField]
    private ActorGenerator _actorGen;

    private Text _text;

    private const string DEAD_TEXT = "RIP: ";
    private const string ESCAPED_TEXT = "Escaped: ";

    protected override void Awake() 
    {
        base.Awake();
        _text = transform.Find("Text").GetComponent<Text>();

        SetVisible(false);
    }

    public override void SetVisible(bool isVisible)
    {
        base.SetVisible(isVisible);
        if(isVisible)
        {
            var actors = _actorGen.GetPlayers();
            string escapeNames = string.Join(", ", actors.Where(act => act.Item1.Status == Status.Escaped).Select(act => act.Item2));
            if (escapeNames == "") escapeNames = "no one!";

            string deadNames = string.Join(", ", actors.Where(act => act.Item1.Status == Status.Dead).Select(act => act.Item2));
            if (deadNames == "") deadNames = "no one!";

            _text.text = $"{ESCAPED_TEXT}{escapeNames}\n{DEAD_TEXT}{deadNames}";
        }
    }
}
