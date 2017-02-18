import java.io._

import scala.annotation.tailrec
import scala.collection.mutable
import scala.io.Source

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
      src.close()
    }
  }

  // Standard code above, custom code below

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {
    val line = lines.next()
    val nk = line.split(" ").map(_.toInt)
    val (n, k) = (nk(0), nk(1))
    writeSolution(bw, n, k)
    /*
    val soln = solve(n, k)
    bw.write(soln.mkString(" "))
    */
    bw.newLine()
  }

  def writeSolution(bw: BufferedWriter, n: Int, k: Int): Unit = {
    val isVisited = new Array[Boolean](k)
    isVisited(0) = true

    @tailrec
    def writeVertices(currTotal: Long, currX: Int): Unit = {
      if (currX != 0) {
        if (currX < k) {
          isVisited(currX) = true
        }
        val linesCrossed: Int = ((currX + 1) to (currX + k - 1)).map { (vertex: Int) =>
          if (isVisited((vertex % n) % k)) {
            if (vertex == n) 1 else 2
          }
          else 0
        }.sum
        val newTotal = currTotal + linesCrossed
        bw.write(newTotal.toString)
        bw.write(" ")
        writeVertices(newTotal, (currX + k) % n)
      }
    }

    writeVertices(1L, k)
  }

  /*
  def solve(n: Int, k: Int): IndexedSeq[BigInt] = {
    val isVisited = mutable.Set[Int]()

    def countSectionsAfterNextLine(currTotal: BigInt, x : Long): BigInt = {
      val currX = (x % n).toInt
      isVisited.add(currX)
      val newX = currX + k
      val linesCrossed: Int = ((currX + 1) to (newX - 1)).map { (vertex: Int) =>
          if (isVisited(vertex % n)) {
            if (vertex == n) 1 else 2
          }
          else 0
      }.sum
      currTotal + 1 + linesCrossed
    }

    val answers = (0L until (n.toLong * k) by k).scanLeft(BigInt(1))(countSectionsAfterNextLine)
    answers.drop(1)
  }
  */
}
