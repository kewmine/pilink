<script lang='ts'>
    let ENDPOINT = '', link_id = '', message = '', status = '';

    async function get_hits() {
        link_id = link_id.trim();

        if (link_id === '') {
            message = 'id cannot be blank x_x';
            return 1;
        }

        ENDPOINT = `http://localhost:8080/${link_id}/expand`;

        try {
            const response = await fetch(ENDPOINT, {
                method: 'GET',
                cache: 'no-cache'
            });
            message = await response.json();
        } catch (err) {
            status = 'error';
            message = `something went wrong x_x`;
        }
    }
</script>

<form on:submit|preventDefault={get_hits}>
    <label>
        <input name='id' placeholder='enter link id below to expand its url' type='text' bind:value={link_id}>
    </label>

    <input type='submit' value='EXPAND LINK'>
</form>
<div class='{status}'> {message} </div>
