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
    def loop(minGames: Long, counts: List[Long]): Long = counts match {
      case 0 :: tl           => loop(minGames + 1, tl)
      case 1 :: Nil          => minGames
      case a :: Nil          => loop(minGames, List(a - 2, 1))
      case 1 :: 1 :: Nil     => minGames + 2
      case 1 :: 1 :: c :: tl => loop(minGames + 2, (c + 1) :: tl)
      case a :: 1 :: tl      => loop(minGames, (a - 2) :: 2l :: tl)
      case a :: b :: Nil     => loop(minGames, List(a - 1, b - 1, 1))
      case a :: b :: c :: tl => loop(minGames, (a - 1) :: (b - 1) :: (c + 1) :: tl)
      case Nil => 0
    }

    val maxGamesPlayed = loop(0, List(n))
    maxGamesPlayed
  }
}
