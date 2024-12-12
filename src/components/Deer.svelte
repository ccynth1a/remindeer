<script lang="ts">
  let { deer, deers } = $props()
  let isShrinking = $state(false);

  const removeDeer = () => {
    isShrinking = true;
    setTimeout(() => {
      deers.update(current => {
        return current.filter(compare => compare.title != deer.title);      
      })                                                                    
    }, 200);
  }
</script>

<div class="deer" class:is-shrinking={isShrinking}>
  <div class="details">
    <h2>{deer.title}</h2>
    <p>Due: {deer.date}</p>
    <p>Urgency: {deer.urgency}</p>
  </div>
  <input type="checkbox" onchange={removeDeer}>
</div>
<style>
.deer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background-color: #fff;
  border: 2px solid #e0c9b3;
  border-radius: 10px;
  padding: 15px;
  margin-bottom: 10px;
  box-shadow: 0 2px 5px rgba(0,0,0,0.1);
  transition: transform 0.2s ease, opacity 0.2s ease;
}

.deer.is-shrinking {
  transform: scale(0);
  opacity: 0;
}

.deer .details {
  flex-grow: 1;
  margin-right: 10px;
}

.deer h2 {
  margin: 0;
  font-size: 14px;
  color: #a3876b;
}

.deer input[type="checkbox"] {
  width: 40px;
  height: 40px;
}
</style>
