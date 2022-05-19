<script lang="ts">
    import logo from '../../assets/meow.png'
    import { err } from '../../store'
    let error_msg: string = ""
    err.subscribe((error: string) => {
        error_msg = error
    })
</script>
<main>
    <img src={logo} alt="Logo" class="main-logo"/>
    {#if error_msg != "" }
        <div class="alert alert-danger">
            {error_msg}
        </div>
    {/if}
</main>
