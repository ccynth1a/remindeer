<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  
  import Header from "../components/Header.svelte";
  import Deer from "../components/Deer.svelte"
  
  // NOTE: SO FAR THERE HAS BEEN A TON OF ISSUES WITH WORKING WITH THE FILESYSTEM. THIS WILL BE FIXED..AT SOME POINT
  let showForm = writable<boolean>(false);
  let title = writable<string>("Eat Grass");
  let date = writable<string>("");
  let urgency = writable<string>("");
  
  interface Deer_t {
    title: string;
    date: string;
    urgency: string;
    completed: boolean;
  }

  let deers = writable<Deer_t[]>([]);
  
  const handleSubmit = async () => {
    try { 
      // File System Logic Here
      //const appDirectory = await appDataDir();
      //const filePath = `${appDirectory}/data.json`;
      //await writeFile({path: filePath, contents: JSON.stringify($deers)})

      const newDeer: Deer_t = {
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
  
  const loadDeerFromDisk = async (): Promise<Deer_t[]> => {
    try {
      //const appDirectory = await appDir();
      //const filePath = `${appDirectory}/data.json`;

      //if (await exists(filePath)) {
      //  const data = await readFile(filePath)
      //  return JSON.parse(data)
      //}
      // return example deer if no file is found
      return [{
        title: "Eat Grass",
        date: "All The Time",
        urgency: "High",
        completed: false,
      }]
    } catch (error) {
      console.error("ERROR READ ", error);
      return [{
        title: "Eat Grass",
        date: "All The Time",
        urgency: "high",
        completed: false,
      }]
    }
  }
  
  onMount(async () => {
    deers.set(await loadDeerFromDisk())
  })

</script>

<Header />
<div class="content">
  {#each $deers as deer}
    <Deer {deer} />
  {/each}
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
          <select bind:value={$urgency}>
            <option value="low">Low</option>
            <option value="medium">Medium</option>
            <option value="high">High</option>
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
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #5a3d2b;
  background-color: #fef4e8;

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
  background-color: #f7c6a4;
  border: none;
  border-radius: 60%;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 2px 5px rgba(0,0,0,0.2);
  cursor: pointer;
  font-size: 32px;
  color: #fff;
}

.add-deer:hover {
  background-color: #e5b094
}

/* Overlay Styling */
.overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0,0,0,0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.form-container {
  background-color: #fff;
  padding: 20px;
  border-radius: 10px;
  box-shadow: 0 2px 5px rgba(0,0,0,0.3);
  width: 90%;
  max-width: 400px;
  text-align: center;
}

.form-container h2 {
  margin-bottom: 20px;
  font-size: 24px;
  color: #5a3d2b;
}

.form-container label {
  display: block;
  margin-bottom: 10px;
  text-align: left;
  font-size: 14px;
  color: #5a3d2b;
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
}

.form-container .button-group {
  display: flex;
  justify-content: space-between;
}

.form-container button {
  background-color: #f7c6a4;
  color: #fff;
  border: none;
  padding: 10px 20px;
  border-radius: 5px;
  cursor: pointer;
}

.form-container button:hover {
  background-color: #e5b094;
}
</style>
