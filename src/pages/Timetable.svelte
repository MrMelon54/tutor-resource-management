<script lang="ts">
  let d: Date = new Date();
  let month: number = d.getMonth();
  let year: number = d.getFullYear();
  let cal = createCalendar(new Date(year, month, 1));

  interface Row {
    id: number;
    startOfWeekDate: Date;
    cells: Cell[];
  }

  interface Cell {
    id: number;
    date: Date;
  }

  function createCalendar(d: Date): Array<Row> {
    console.log("DOW:", d.getDay());
    let a = new Array(6).map((_, id): Row => ({id, startOfWeekDate: null, cells: new Array(7)}));
    let z = 0;
    for (let i = 0; i < a.length; i++) {
      console.log(a[i]);
      for (let j = 0; j < a[i].cells.length; j++) {
        a[i].cells[j] = {id: z, date: null};
        z++;
      }
    }
    return a;
  }

  function numberCalendar(a: Array<Array<7>>) {}
</script>

<table>
  <tr>
    <th>Mon<span>day</span></th>
    <th>Tue<span>sday</span></th>
    <th>Wed<span>nesday</span></th>
    <th>Thu<span>rsday</span></th>
    <th>Fri<span>day</span></th>
    <th>Sat<span>urday</span></th>
    <th>Sun<span>day</span></th>
  </tr>
  {#each cal as c (c.row)}{/each}
</table>

<style lang="scss">
  table {
    table-layout: fixed;
    border-collapse: collapse;
    border: 1px solid grey;
    width: 100%;
    height: 100%;

    tr {
      border: 1px solid grey;

      td,
      th {
        border: 1px solid grey;
      }
    }
  }
</style>
