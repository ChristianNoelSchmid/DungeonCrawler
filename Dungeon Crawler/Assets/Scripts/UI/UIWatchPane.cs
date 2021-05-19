using DungeonCrawler.Monobehaviours;
using System;
using System.Linq;
using UnityEngine;
using UnityEngine.UI;

public class UIWatchPane : UIGroup
{
    [SerializeField]
    private ActorGenerator _actorGen;
    [SerializeField]
    private CameraMovement _cameraMovement;

    private Text _statusText;
    private Text _watchText;

    private const string DEAD_TEXT = "You're dead!";
    private const string ESCAPED_TEXT = "You escaped!";
    private Color _deadColor = Color.red;
    private Color _escapedColor = Color.green;

    private int _actorIndex = 0;

    protected override void Awake() 
    {
        base.Awake();
        _statusText = transform.Find("StatusText").GetComponent<Text>(); 
        _watchText = transform.Find("WatchText").GetComponent<Text>();

        SetVisible(false);
    }

    public void OnStatusChange(bool died)
    {
        _statusText.text = died ? DEAD_TEXT : ESCAPED_TEXT;
        _statusText.color = died ? _deadColor : _escapedColor;
         
        SetVisible(true);
    }
    public void OnNext()
    {
        var actors = _actorGen.GetPlayers();
        if (actors.Count == 0) return;

        _actorIndex += 1;
        if (_actorIndex >= actors.Count) _actorIndex = 0;

        WatchActor(actors[_actorIndex].Item1.transform, actors[_actorIndex].Item2);
    }

    public void OnPrevious()
    {
        var actors = _actorGen.GetPlayers();
        if (actors.Count == 0) return;

        _actorIndex -= 1;
        if (_actorIndex < 0) _actorIndex = actors.Count - 1;

        WatchActor(actors[_actorIndex].Item1.transform, actors[_actorIndex].Item2);
    }

    public void WatchActor(Transform actorTransform, string name)
    {
        _cameraMovement.Target = actorTransform;
        _watchText.text = "You are watching: " + name;
    }
}
