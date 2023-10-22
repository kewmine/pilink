<script lang='ts'>
    import '../global.scss';
    import Footer from "$lib/modules/footer.svelte"
    import Header from "$lib/modules/header.svelte"
</script>

<div class='section'>
    <Header/>
    <slot/>
</div>

<Footer/>

<style lang='scss'>






</style>
