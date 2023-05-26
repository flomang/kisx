<script lang="ts">
    export let total_lots = 0;
    export let estimated_value = 0;
    export let for_sale = 0;
    export let pending = 0;
</script>

<div class="content">
    <div class="value-container">
        <div>Estimated Value:</div>
        <div>
            <b>₿</b> {estimated_value} 
        </div>
    </div>
    <div class="value-container">
        <div>Total:</div>
        {total_lots} 
    </div>
    <div class="value-container">
        <div>For Sale:</div>
        {for_sale}
    </div>
    <div class="value-container">
        <div>Pending Trade:</div>
        {pending}
    </div>
</div>

<style>
    .content {
        height: 100%;
        width: 100%;
    }
    .value-container {
        display: flex;
        justify-content: space-between;
    }
</style>
