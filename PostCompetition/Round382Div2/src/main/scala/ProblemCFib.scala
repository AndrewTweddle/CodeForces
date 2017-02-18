import java.io._
import scala.io.Source

import scala.annotation.tailrec

object ProblemCFib {
  def main(args: Array[String]): Unit = processStdInOut()

  def processStdInOut(): Unit = {
    val src = Source.fromInputStream(System.in)
    processFromSource(src)
  }

  def processFromSource(src: Source): Unit = {
    try {
      val bw = new BufferedWriter(new OutputStreamWriter(System.out));
      try {
        val lines = src.getLines()
        processLines(lines, bw)
      } finally {
        bw.flush()
      }
    } finally {
      src.close();
    }
  }

  // Standard code above, custom code below
  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {
    val n = lines.next().toLong
    val soln = solve(n)
    bw.write(soln.toString)
    bw.newLine()
  }

  // Cache the Fibonacci lookup for good performance:
  var minPlayersByChampionsWinsLookup = Map[Long, Long](
    0L -> 1L,  // A single player is champion without playing a single match
    1L -> 2L   // With 2 players the champion wins after a single match
  )

  // The following function is equivalent to fib(wins + 2) where fib is the Fibonacci sequence
  def minPlayersForChampionWithWins(wins: Long): Long = {
    if (minPlayersByChampionsWinsLookup.contains(wins)) {
      minPlayersByChampionsWinsLookup(wins)
    } else {
      val result = minPlayersForChampionWithWins(wins - 1) + minPlayersForChampionWithWins(wins - 2)
      minPlayersByChampionsWinsLookup += wins -> result
      result
    }
  }

  def solve(n: Long): Long = {
    @tailrec
    def search(candidateWins: Long): Long = {
      val minTournamentSize = minPlayersForChampionWithWins(candidateWins)

      if (minTournamentSize == n) candidateWins
      else if (minTournamentSize > n) (candidateWins - 1)  // overshot the target, so use previous value
      else search(candidateWins + 1)
    }

    search(0)
  }
}
