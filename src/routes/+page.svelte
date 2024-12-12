<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { derived, writable } from "svelte/store";
  
  import Header from "../components/Header.svelte";
  import Deer from "../components/Deer.svelte"
  
  // NOTE: SO FAR THERE HAS BEEN A TON OF ISSUES WITH WORKING WITH THE FILESYSTEM. THIS WILL BE FIXED..AT SOME POINT
  let showForm = writable<boolean>(false);
  let title = writable<string>("Eat Grass");
  let date = writable<string>("");
  let urgency = writable<string>("");

  let sortType = writable<string>("title")
  
  interface Deer_t {
    id: number,
    title: string;
    date: string;
    urgency: string;
    completed: boolean;
  }

  export const deers = writable<Deer_t[]>([]);
  
  export let sortedDeers = derived([deers, sortType], ([$deers, $sortType]) => {
    return [...$deers].sort((a,b) => {
      if ($sortType == 'title') return a.title.localeCompare(b.title);
      if ($sortType == 'urgency') return a.urgency.localeCompare(b.urgency);
      if ($sortType == 'date') return new Date(a.date) - new Date(b.date);
    })
  })

  const handleSubmit = async () => {
    try { 
      const newDeer: Deer_t = {
        id: $deers.length + 1,
        title: $title,
        date: $date,
        urgency: $urgency,
        completed: false,
      }
      deers.update( current => [...current, newDeer])

      title.set("");
      date.set("");
      urgency.set("");
      
      showForm.set(false);
    } catch (error) {
      console.error("ERROR WRITE: ", error)
    }
  }
  
  const loadDeerFromDisk = async () => {
    console.log("Grabbing Deer From Disk")
    let deers: string = await invoke('get_items');
    let deers_json = JSON.parse(deers)
    console.log("Deer",deers_json)
    return deers_json
  }

  onMount(async () => {
    deers.set(await loadDeerFromDisk())
  })

  export const writeDeerToDisk = async () => {
    deers.subscribe(async (data) => {
      await invoke('write_items', { data: JSON.stringify(data) })
    })
  }
  
</script>

<Header />
<div class="content">
  <div class="sort-by">
    <label for="sort">Sort by:</label>
    <select id="sort" bind:value={$sortType}>
      <option value="title">Title</option>
      <option value="urgency">Urgency</option>
      <option value="date">Date</option>
    </select> 
  </div>

  {#each $sortedDeers as deer (deer.id)}
    <Deer {deer} {deers}/>
  {/each}
  <button class="save-deer" onclick={writeDeerToDisk}>🖫</button>
</div>

<button onclick={() => showForm.set(true)} class="add-deer" aria-label="Add Deer">+</button>

{#if $showForm}
  <div class = "overlay">
    <div class="form-container">
      <h2>Add New Task</h2>
      <form onsubmit={handleSubmit}>
        <label>
          Title:
          <input
            type="text"
            bind:value={$title}
            placeholder="Enter Task Title"
            required
          />
        </label>

        <label>
          Date:
          <input 
            type="date"
            bind:value={$date}
            required
          />
        </label>

        <label>
          Urgency:
          <select required bind:value={$urgency}>
            <option value="Low">Low</option>
            <option value="Medium">Medium</option>
            <option value="High">High</option>
          </select>
        </label>

        <div class="button-group">
          <button type="submit">Add Task</button>
          <button type="button" onclick={() => showForm.set(false)}>
            Cancel
            </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
/* Boilerplate */
:root {
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  --bg-color: #fef4e8;
  --text-color: #5a3d2b;
  --form-bg: #fff;
  --button-bg: #f7c6a4;
  --button-hover-bg: #e5b094;
  --overlay-bg: rgba(0,0,0,0.5);
  
  background-color: var(--bg-color);
  color: var(--text-color);
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  transition: background-color 0.3s ease, color 0.3s ease;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.content {
  margin-top: 80px;
  padding: 20px;
}

/* Add Item Button */
.add-deer {
  position: fixed;
  bottom: 20px;
  right: 20px;
  width: 60px;
  height: 60px;
  background-color: var(--button-bg);
  border: none;
  border-radius: 60%;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 2px 5px rgba(0,0,0,0.2);
  cursor: pointer;
  font-size: 32px;
  color: #fff;
  transition: background-color 0.3s ease;
}

.save-deer {
  position: fixed;
  bottom: 20px;
  left: 20px;
  height: 60px;
  background-color: var(--button-bg);
  border: none;
  padding: 20px;
  width: 60px;
  height: 60px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 2px 5px rgba(0,0,0,0.2);
  cursor: pointer;
  font-size: 24px;
  color: #fff;
  transition: background-color 0.3s ease;
  font-weight: bold;
}

.sort-by {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background-color: #fff;
  border: 2px solid #e0c9b3;
  border-radius: 10px;
  padding: 15px;
  margin-bottom: 10px;
  box-shadow: 0 2px 5px rgba(0,0,0,0.1);
}

.add-deer:hover {
  background-color: var(--button-hover-bg);
}

/* Theme Toggle Styling */ 
.toggle-theme {
  position: fixed;
  bottom: 20px;
  left: 20px;
  background-color: var(--button-bg);
  color: #fff;
  border: none;
  padding: 10px 15px;
  border-radius: 5px;
  cursor: pointer;
  font-size: 14px;
  box-shadow: 0 2px 5px rgba(0,0,0,0.2);
  transition: background-color 0.3s ease;
}

.toggle-theme:hover {
  background-color: var(--button-hover-bg);
}

/* Overlay Styling */
.overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: var(--overlay-bg);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.form-container {
  background-color: var(--form-bg);
  padding: 20px;
  border-radius: 10px;
  box-shadow: 0 2px 5px rgba(0,0,0,0.3);
  width: 90%;
  max-width: 400px;
  text-align: center;
  transition: background-color 0.3s ease;
}

.form-container h2 {
  margin-bottom: 20px;
  font-size: 24px;
  color: var(--text-color);
}

.form-container label {
  display: block;
  margin-bottom: 10px;
  text-align: left;
  font-size: 14px;
  color: var(--text-color);
}

.form-container input, .form-container select {
  width: 100%;
  padding: 8px;
  margin-top: 5px;
  margin-bottom: 20px;
  border: 1px solid #ccc;
  border-radius: 5px;
  font-size: 14px;
  font-family: inherit;
  box-sizing: border-box;
  background-color: var(--bg-color);
  color: var(--text-color);
  transition: background-color 0.3s ease, color 0.3s ease;
}

.form-container .button-group {
  display: flex;
  justify-content: space-between;
}

.form-container button {
  background-color: var(--button-bg);
  color: #fff;
  border: none;
  padding: 10px 20px;
  border-radius: 5px;
  cursor: pointer;
}

.form-container button:hover {
  background-color: var(--button-hover-bg);
}
</style>
