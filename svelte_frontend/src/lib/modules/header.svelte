<script>
    import pilogo from "$lib/assets/pi.png";
</script>

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
            <li><a href='/expand'> expand link </a></li>
        </ul>
    </div>
</div>

<style lang="scss">
  .display {
    display: flex;
    flex-direction: column;
    padding: 10vh 0 5vh 0;
    -webkit-user-select: none; /* Safari */
    -ms-user-select: none; /* IE 10 and IE 11 */
    user-select: none; /* Standard syntax */
  }
  .name {
    color: white;
  }

  .summary {
    color: #a4a0a0;
  }

  .navbar {
    font-size: 1.8vh;
    display: flex;
    flex-direction: row;
    justify-content: center;

  }

  a {
    color: #c0b7b7;
    text-decoration: none;
  }

  a:active, a:hover {
    color: white;
    text-decoration: none;
  }

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

  img {
    -webkit-user-drag: none;
    -khtml-user-drag: none;
    -moz-user-drag: none;
    -o-user-drag: none;
    user-drag: none;
  }
</style>