<script>
	export let onClick;
	export let disabled = false;
</script>

<button
	class="relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 hover:text-white focus:ring-4 focus:outline-none focus:ring-blue-300 disabled:cursor-not-allowed"
	on:click={onClick}
	{disabled}
>
	<span
		class="relative transition-all ease-in duration-75 bg-white rounded-md group-hover:bg-opacity-0"
	>
		<slot />
	</span>
</button>
