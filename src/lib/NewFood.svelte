<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { foodsNormalized } from './store.js';
	import { _ } from 'svelte-i18n'; 
	import MaterialFloatingLabel from './MaterialFloatingLabel.svelte';
	import MaterialFloatingLabelError from './MaterialFloatingLabelError.svelte';
	import SvgOk from './SvgOk.svelte';
	import SvgCancel from './SvgCancel.svelte';
	import SvgAdd from './SvgAdd.svelte';

	let active = false; // track if the button is clicked (new food is being added by the user)

	// initialize new food to be filled in with data provided by the user
	let newFood = {
		id: 0,
		name: '',
		serving_size: '',
		unit: '',
		normalized_calories: '',
		normalized_protein: '',
		normalized_carbohydrate: '',
		normalized_fat: ''
	};

	// validation error object
	let validationError = {};

	function validateNewFood(newFood) {
		if (!newFood.name.trim()) {
			validationError.name = $_('error.noFoodName');
		}
		if (newFood.serving_size <= 0) {
			validationError.serving_size = $_('error.invalidServingSize');
		}
		if (!newFood.unit) {
			validationError.unit = $_('error.noUnit');
		}
		if (!newFood.normalized_protein || Number(newFood.normalized_protein) < 0) {
			validationError.normalized_protein = $_('error.negativeAmount');
		}
		if (!newFood.normalized_protein || Number(newFood.normalized_carbohydrate) < 0) {
			validationError.normalized_carbohydrate = $_('error.negativeAmount');
		}
		if (!newFood.normalized_fat || Number(newFood.normalized_fat) < 0) {
			validationError.normalized_fat =  $_('error.negativeAmount');
		}
		if (!newFood.normalized_calories || Number(newFood.normalized_calories) < 0) {
			validationError.normalized_calories = $_('error.negativeCalories');
		}
		return Object.keys(validationError).length === 0;
	}

	async function addNewFood() {
		// Make a validation check
		if (validateNewFood(newFood)) {
			// Invoke backend code to add the new food to the database
			await invoke('add_new_food_normalized', { newFood });

			// Clear the input fields and reset the newFood object
			newFood = {
				id: 0,
				name: '',
				serving_size: 0,
				unit: '',
				normalized_calories: 0,
				normalized_protein: 0,
				normalized_carbohydrate: 0,
				normalized_fat: 0
			};

			// Refresh the list of foods
			foodsNormalized.set(await invoke('get_foods_normalized'));
			active = false;
		}
	}
</script>

<div class="flex flex-col items-center">
	<div class="mt-4 mb-4">
		<button on:click={() => (active = !active)} class="text-button">
			{#if !active}
				<SvgAdd /> {$_('foods.newFood')}
			{:else}
				<SvgCancel /> {$_('cancel')}
			{/if}
		</button>
	</div>

	{#if active}
		<div class="block tracking-tighter text-sm">
			<table class="mx-auto">
				<tr>
					{#if !validationError.name}
						<td colspan="2"
							><MaterialFloatingLabel label={$_('foods.foodName')} bind:value={newFood.name} type="text" /></td
						>
					{:else}
						<td colspan="2"
							><MaterialFloatingLabelError
								error={$_('foods.noName')}
								errorMessage={validationError.name}
								bind:value={newFood.name}
								type="text"
							/></td
						>
					{/if}
				</tr>

				<tr>
					{#if !validationError.serving_size}
						<td>
							<MaterialFloatingLabel
								label={$_('servingSize')}
								bind:value={newFood.serving_size}
								type="number"
							/>
						</td>
					{:else}
						<td>
							<MaterialFloatingLabelError
								error={$_('foods.noServingSize')}
								errorMessage={validationError.serving_size}
								bind:value={newFood.serving_size}
								type="number"
							/>
						</td>
					{/if}
					{#if !validationError.unit}
						<td>
							<MaterialFloatingLabel
								label={$_('unit')}
								bind:value={newFood.unit}
								type="text"
							/>
						</td>
					{:else}
						<td>
							<MaterialFloatingLabelError
								error={$_('foods.noUnit')}
								errorMessage={validationError.unit}
								bind:value={newFood.unit}
								type="text"
							/>
						</td>
					{/if}
				</tr>

				<tr>
					{#if !validationError.normalized_calories}
						<td>
							<MaterialFloatingLabel
								label="{$_('calories')} (kcal)"
								bind:value={newFood.normalized_calories}
								type="number"
							/>
						</td>
					{:else}
						<td>
							<MaterialFloatingLabelError
								bind:value={newFood.normalized_calories}
								error={$_('foods.invalidAmount')}
								errorMessage={validationError.normalized_calories}
								type="number"
							/>
						</td>
					{/if}

					{#if !validationError.normalized_protein}
						<td>
							<MaterialFloatingLabel
								label="{$_('foods.protein')} (g)"
								bind:value={newFood.normalized_protein}
								type="number"
							/>
						</td>
					{:else}
						<td>
							<MaterialFloatingLabelError
								bind:value={newFood.normalized_protein}
								error={$_('foods.invalidAmount')}
								errorMessage={validationError.normalized_protein}
								type="number"
							/>
						</td>
					{/if}
				</tr>

				<tr>
					{#if !validationError.normalized_carbohydrate}
						<td>
							<MaterialFloatingLabel
								label="{$_('foods.carbohydrate')} (g)"
								bind:value={newFood.normalized_carbohydrate}
								type="number"
							/>
						</td>
					{:else}
						<td>
							<MaterialFloatingLabelError
								bind:value={newFood.normalized_carbohydrate}
								error={$_('foods.invalidAmount')}
								errorMessage={validationError.normalized_carbohydrate}
								type="number"
							/>
						</td>
					{/if}

					{#if !validationError.normalized_fat}
						<td>
							<MaterialFloatingLabel
								label="{$_('foods.fat')} (g)"
								bind:value={newFood.normalized_fat}
								type="number"
							/>
						</td>
					{:else}
						<td>
							<MaterialFloatingLabelError
								bind:value={newFood.normalized_fat}
								error={$_('foods.invalidAmount')}
								errorMessage={validationError.normalized_fat}
								type="number"
							/>
						</td>
					{/if}
				</tr>
			</table>
		</div>
		<div>
			<button on:click={addNewFood} class="text-button">
				<SvgOk /> OK
			</button>
		</div>
	{/if}
</div>
