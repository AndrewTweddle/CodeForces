import java.io._
import scala.io.Source
import scala.collection.mutable.BitSet

object ProblemD {
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
    val line = lines.next()
    val nk = line.split(" ").map(_.toInt)
    val (n, k) = (nk(0), nk(1))
    val soln = solve(n, k)
    bw.write(soln.mkString(" "))
    bw.newLine()
  }

  def solve(n: Int, k: Int): IndexedSeq[BigInt] = {
    val joinedVertices = Array.tabulate[BitSet](n)(_ => new BitSet())

    def countSectionsAfterNextLine(currTotal: BigInt, x : BigInt): BigInt = {
      val currX = x % n
      val newX = (x + k) % n
      val linesCrossed = (
        for {
          v1 <- (currX + 1) until (currX + k) map (_.toInt % n)
          v2 <- (currX + k + 1) until (currX + n) map (_.toInt % n)
        } yield if (joinedVertices(v1).contains(v2)) BigInt(1) else BigInt(0)
      ).sum
      joinedVertices(currX.toInt).add(newX.toInt)
      joinedVertices(newX.toInt).add(currX.toInt)
      currTotal + 1 + linesCrossed
    }

    val answers = (BigInt(0) until BigInt(n * k) by k).scanLeft(BigInt(1))(countSectionsAfterNextLine)
    answers.drop(1)
  }
}
