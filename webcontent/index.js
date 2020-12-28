const $ = (id) => document.getElementById(id);

const render = (r, d) => {
  const t = document.createElement('div');
  t.innerHTML = d;
  while (t.firstChild) r.appendChild(t.firstChild);
}

const AVATAR = (function() {
  updateUIState = () => {
    const buttons = ['', '', '', '', '', '', '', '', '', '', '', '', '', '', ''];
    buttons.forEach(b => render($('buttons'), `<div class="button"></div>`));
  }

  onButtonClick = () => {
    fetch('/keys', {
      method: 'POST',
      body: JSON.stringify({ keys: '{+CTRL}a{-CTRL}' })
    }).then(function(response) {
      return response.json();
    })
  }

  init = () => {
    this.updateUIState();
    $('buttons').addEventListener('click', (e) => {
      this.onButtonClick();
    });
  }

  init();
}());

/*

*/


/*
base - create folder
base - timer (seconds) with sounds select
base - play sound (second press - stop/restart/overlap)
time - simple display
time - countdown
time - timer
system - open website (background)
system - open app/file
system - type message (enter option at the end)
system - hotkey (record hotkeys)
(https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-appcommand)
multimedia - play/pause
multimedia - mute
multimedia - next/prev track
multimedia - decrease/increase volume
youtube - viewers count
youtube - send message (200 chars)
twitch - viewers count
twitch - send message (500 chars)
twitch - stream/game title
twitch - mode (all/followers/subs)
twitch - create clip
twitch - slow chat
twitter - change name
twitter - tweet (280 chars)
mixer - send message (280 chars)
mixer - viewers count
mixer - stream/game title

obs studio -
streamlabs -
streamlabs obs -
xsplit -

? - mic mute/unmute
? - discord mic mute/unmute

*/
