import java.io._
import scala.io.Source

import scala.annotation.tailrec

object ProblemC {
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

  def solve(n: Long): Long = {
    @tailrec
    def loop(minGames: Int, countsByNumberOfWins: List[Long]): Option[Long] = {
      if (countsByNumberOfWins.head != 0) {
        val fullList: List[Long] = List.fill[Long](minGames)(0L) ++ countsByNumberOfWins
        println(fullList.mkString(", "))
      }
      countsByNumberOfWins match {
        case 0 :: tl           => loop(minGames + 1, tl)
        case 1 :: Nil          => Some(minGames)
        case 1 :: 1 :: Nil     => loop(minGames + 2, 1L :: Nil)

        // Only allow uneven matches (where one player has played an extra game):
        case a :: Nil          => None
        case a :: 0 :: tl      => None

        // Reject if there are more "a" players than "b", so that we don't have to decide how many to eliminate
        // (since that decision must be made up-front):
        case a :: b :: tl if a > b  => None

          // Have all the "a" players lost to that many "b" players (who increase their number of wins):
        case a :: b :: Nil     => loop(minGames + 1, List(b - a, a))
        case a :: b :: c :: tl => loop(minGames + 1, (b - a) :: (c + a) :: tl)

        case _  => None
      }
    }

    def runTournament(matchesWhereBothPlayersHavePlayedNoGames: Long,
                      playersWithNoGamesWhoWillLoseToAPlayerWithOneGame: Long): Long = {
      val playersWithNoGamesWhoEliminateAPlayerWithOneWin =
        n - 2 * matchesWhereBothPlayersHavePlayedNoGames - playersWithNoGamesWhoWillLoseToAPlayerWithOneGame
      if (matchesWhereBothPlayersHavePlayedNoGames == 0 && playersWithNoGamesWhoEliminateAPlayerWithOneWin > 0) {
        0
      } else {
        val msg =
          s"""Attempt a tournament with $n players and the following initial results:
             |  a) $matchesWhereBothPlayersHavePlayedNoGames matches where both players haven't played yet.
             |  b) $playersWithNoGamesWhoEliminateAPlayerWithOneWin matches where a player with 0 games beats a player with 1 win.
             |  c) $playersWithNoGamesWhoWillLoseToAPlayerWithOneGame players who haven't played yet (but will lose to players with 1 game).
           """
        println(msg.stripMargin)
        val playersByNumberOfWins =
          List(playersWithNoGamesWhoWillLoseToAPlayerWithOneGame, matchesWhereBothPlayersHavePlayedNoGames)
        val maxGamesOption = loop(0, playersByNumberOfWins)
        val maxPlayed = maxGamesOption.getOrElse(0L)
        if ( maxGamesOption.isDefined) {
          println(s"Max games played: $maxPlayed")
        } else {
          println("No solution found")
        }
        println()
        maxPlayed
      }
    }

    // Look for a solution where only in the first round do people with the same number of wins (zero) play each other.
    // Thereafter expect all matches to be lopsided with the better player having more wins:
    val tournaments = for {
      numEvens <- 0L to n/2
      numUnevens <- 0L to n - 2 * numEvens
    } yield runTournament(numEvens, numUnevens)
    val maxGamesPlayed = tournaments.max
    maxGamesPlayed
  }
}
