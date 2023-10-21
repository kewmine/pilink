<script lang='ts'>
  let uri = '', count_hits = true, message = '', status = '';

  function toggle_hits() {
    count_hits = !count_hits;
    console.log('count_hits');
  }

  let ENDPOINT = '/new';

  async function create_link() {
    // sanitize
    uri = uri.trim();
    if (uri === '') {
      status = 'error';
      message = 'uri cannot be empty';
      return 1;
    }

    try {
      // post body
      let data = new URLSearchParams();
      data.append('uri', uri);
      data.append('count_hits', count_hits.toString());

      // send request to backend
      const response = await fetch(ENDPOINT, {
        method: 'POST',
        body: data
      });
      status = 'success';
      message = await response.text();

    } catch (err) {
      status = 'error';
      message = 'something went wrong x_x';
    }
  }
</script>

<form on:submit|preventDefault={create_link}>
  <label>
    <input name='uri' placeholder='url to be shortened goes here' type='text' bind:value={uri}>
  </label>

  <div>
    count hits:
    <button on:click|preventDefault={toggle_hits}> {count_hits}</button>
  </div>


  <input type='submit' value='CREATE'>
</form>
<div class='{status}'> {message} </div>

<style>
    button {
        background-color: #2f2f2f;
        border: none;
        color: #ffffff;
        padding: 0.5vh 1vh 0.5vh 1vh;
        text-align: center;
        text-decoration: none;
        display: inline-block;
        font-size: 2vh;
    }
</style>