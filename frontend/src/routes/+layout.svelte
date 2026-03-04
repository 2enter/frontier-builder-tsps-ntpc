<script lang="ts">
    import '../app.css';
    import {PUBLIC_FA_KIT_URL} from '$env/static/public';

    let {children} = $props();
</script>

<svelte:head>
    {#if PUBLIC_FA_KIT_URL}
        <script src={PUBLIC_FA_KIT_URL} crossorigin="anonymous"></script>
    {/if}
</svelte:head>

{@render children()}
