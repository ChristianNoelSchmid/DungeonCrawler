using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

public class UIStatBar : MonoBehaviour
{
    [SerializeField]
    private int _maxHealth;

    private Image _image;
    private void Awake()
    {
        _image = transform.Find("Background").Find("StatBar").GetComponent<Image>();
        if (_image == null)
            Debug.LogError("Expected Image to be attached to child for UIStatBar");

        SetHealth(_maxHealth);
    }
    
    public void SetHealth(int health) => _image.fillAmount = (float)health / _maxHealth;
}
