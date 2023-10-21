<script lang='ts'>
  import '../global.scss';
  import pilogo from '$assets/pi.png';
</script>

<div class='section'>

  <div class='display'>
    <div class='container'>
      <img src='{pilogo}' alt='logo'>
      <div class='name'>PILINK
        <div class='summary'>suckless link shortener</div>
      </div>
    </div>
    <div class='navbar'>
      <ul>
        <li><a href='/'> create link </a>
        <li><a href='/hits'> check hits </a></li>
      </ul>
    </div>

  </div>
  <slot />
</div>


<div class='footer'>
  <hr>
  <div class='info'>
    © 2023 pil.ink
    <div class='divider'>|</div>
    <a href='https://kew.bio'>kew.bio</a>
  </div>
</div>

<style lang='scss'>
  .display {
    img {
      -webkit-user-drag: none;
      -khtml-user-drag: none;
      -moz-user-drag: none;
      -o-user-drag: none;
      user-drag: none;
    }

    display: flex;
    flex-direction: column;
    padding: 10vh 0 5vh 0;

    .name {
      color: white;
    }

    .summary {
      color: #a4a0a0;
    }

    -webkit-user-select: none; /* Safari */
    -ms-user-select: none; /* IE 10 and IE 11 */
    user-select: none; /* Standard syntax */
  }

  hr {
    align-self: center;
    margin: 2vh 0 2vh 0;
    width: 50vw;
    border: 1px solid #9ad1dc;
  }

  .footer {
    display: flex;
    flex-direction: column;

    width: 100vw;
    padding-top: 10vh;
    color: #c0b7b7;
    font-size: 1.4vh;
  }

  .info {
    display: flex;
    flex-direction: row;
    justify-content: center;
    text-align: center;
  }

  .divider {
    padding: 0 2vh 0 2vh;
  }

  a {
    color: #c0b7b7;
    text-decoration: none;
  }

  a:active, a:hover {
    color: white;
    text-decoration: none;
  }

  .navbar {
    font-size: 1.8vh;
    display: flex;
    flex-direction: row;
    justify-content: center;

    ul {
      padding: 0;

      display: flex;
      align-items: center;
      justify-content: center;
    }

    li {
      padding: 0.5vh 2vh 1vh 2vh;
      border: 1px solid #818181;
      display: inline;
    }

    li:hover, li:active {
      color: white;
      border: 1px solid white;
    }
  }
</style>
