using DungeonCrawler.Networking;
using UnityEngine;
using UnityEngine.UI;

public class UIEnterPane : UIGroup
{ 
    [SerializeField]
    private NetworkDatagramHandler _datagramHandler;

    private InputField _nameField;
    private InputField _ipAddrField;
    private Text _errorText;

    protected override void Awake()
    {
        base.Awake();

        _nameField = transform.Find("NameField").GetComponent<InputField>();
        _ipAddrField = transform.Find("IPAddressField").GetComponent<InputField>();
        _errorText = transform.Find("ErrorText").GetComponent<Text>();

        SetVisible(true);
    }

    public void OnSubmit()
    {
        _errorText.enabled = false;
        if (!_datagramHandler.AttemptSignin(_nameField.text, _ipAddrField.text))
            _errorText.enabled = true;
        else
            SetVisible(false);
    }
}
