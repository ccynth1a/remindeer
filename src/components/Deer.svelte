<script lang="ts">
  let { deer } = $props()
</script>

<div class="deer">
  <div class="details">
    <h2>{deer.title}</h2>
    <p>Due: {deer.date}</p>
  </div>
  <input type="checkbox">
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
