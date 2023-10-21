<script lang='ts'>
  let ENDPOINT = '', link_id = '', message = '', status = '';

  async function get_hits() {
    link_id = link_id.trim();

    if (link_id === '') {
      message = 'id cannot be blank x_x';
      return 1;
    }

    ENDPOINT = `/${link_id}/hits`;

    try {
      const response = await fetch(ENDPOINT, {
        method: 'GET',
        cache: 'no-cache'
      });
      let data = await response.json();

      if (typeof data === 'number') {
        message = `hits: ${data}`;
      } else if (typeof data === 'string') {
        message = `${data}`;
      } else if (typeof data === 'boolean') {
        message = `hits counter is disabled for this id`;
      }
    } catch (err) {
      status = 'error';
      message = 'something went wrong x_x';
    }
  }
</script>

<form on:submit|preventDefault={get_hits}>
  <label>
    <input name='id' placeholder='enter link id below to check its hits' type='text' bind:value={link_id}>
  </label>

  <input type='submit' value='GET HITS'>
</form>

<div class='{status}'> {message} </div>

<style>
</style>