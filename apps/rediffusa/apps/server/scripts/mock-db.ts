import neo4j from "neo4j-driver";
import dotenv from "dotenv";

dotenv.config();

const driver = neo4j.driver(
  process.env.DB_URI || "",
  neo4j.auth.basic(process.env.DB_USER || "", process.env.DB_PASSWORD || "")
);

async function run() {
  const session = driver.session();

  try {
    const result = await session.run(
      `
        CREATE(:Gender {name: 'MALE'}) ;
        CREATE(:Gender {name: 'FEMALE'});
        CREATE(:Gender {name: 'NON_BINARY'});

        CREATE (:AreaType {name: 'country', description: 'A national level jurisdiction currently recognised by CISAC as a valid jurisdiction for the publishing of works.'});

        CREATE (:Area {name: 'Serbia'});
        CREATE (:Area {name: 'Australia'});

        MATCH (a:Area), (b:AreaType)
        WHERE a.name = 'Australia' AND b.name = 'country'
        CREATE (a)-[:IS_AREA_TYPE]->(b);

        MATCH (a:Area), (b:AreaType)
        WHERE a.name = 'Serbia' AND b.name = 'country'
        CREATE (a)-[:IS_AREA_TYPE]->(b);

        CREATE (:ArtistType {name: 'INTERDISCIPLINARY'});
        CREATE (:ArtistType {name: 'MUSIC'});

        CREATE (:Artist {
          name: 'Windbreaker', 
          comment: '5gum in the year 3000',
          ipi: 'random_string',
          isni: 'another_random_string',
          creditCount: 0,
          beginDate: '2023-01-31T11:37:03.877Z'
        });

        CREATE (:Artist {
          name: 'DaGooj', 
          comment: 'HighFrequencyFunkening',
          ipi: 'random_string2',
          isni: 'another_random_string2',
          creditCount: 0,
          beginDate: '2023-01-31T11:37:03.876Z'
        });

        MATCH (a:Artist), (b:ArtistType), (c:Gender), (d:Area)
        WHERE a.name = 'Windbreaker' AND 
        b.name = 'INTERDISCIPLINARY' AND
        c.name = 'MALE' AND
        d.name = 'Australia'
        CREATE (a)-[:IS_ARTIST_TYPE]->(b)
        CREATE (a)-[:HAS_GENDER]->(c)
        CREATE (a)-[:IN_AREA]->(d);

        MATCH (a:Artist), (b:ArtistType), (c:Gender), (d:Area)
        WHERE a.name = 'DaGooj' AND 
        b.name = 'MUSIC' AND
        c.name = 'NON_BINARY' AND
        d.name = 'Serbia'
        CREATE (a)-[:IS_ARTIST_TYPE]->(b)
        CREATE (a)-[:HAS_GENDER]->(c)
        CREATE (a)-[:IN_AREA]->(d);

        CREATE(:Format {name: 'wav', description: 'A WAV file is a lossless audio format that does not compress the original analog audio recording from which it is derived. Microsoft and IBM pioneered the Waveform audio file format, and it is now widely used by digital music companies around the world.'});
        CREATE  (:Format {name: 'mp3', description: 'MP3 (MPEG-1 Audio Layer-3) is a standard technology and format for compressing a sound sequence into a very small file (about one-twelfth the size of the original file) while preserving the original level of sound quality when it is played.'});

        create (:Medium {name: 'vinyl', description: 'A vinyl record is an analog sound storage medium in the form of a flat disc with an inscribed, modulated spiral groove.'})

        CREATE (:Medium {name: 'streaming-media', description: 'Streaming media is video or audio content sent in compressed form over the internet and played immediately over a user device, rather than being saved to the device hard drive or solid-state drive.'});

        CREATE (:Medium {name: 'digital-download', description: 'A digital download is an electronic form of acquiring a document, file or software package. Digital downloads occur over the Internet, a network or a USB device (hard drive or thumb drive) most commonly.'});

        MATCH (a:Medium), (b:Format)
        WHERE a.name = 'streaming-media' OR a.name = 'digital-download' AND
        b.name = 'wav' OR b.name = 'mp3' 
        CREATE (a)-[:IN_FORMAT]->(b);

        CREATE (:ReleaseStatus {name: "UNSCHEDULED_RELEASE"});
        CREATE (:ReleaseStatus {name: "SCHEDULED_RELEASE"});
        CREATE (:ReleaseStatus {name: "RELEASED"});

        CREATE (:ReleaseQuality {name: "LOW"});
        CREATE (:ReleaseQuality {name: "MEDIUM"});
        CREATE(:ReleaseQuality {name: "HIGH"});
      `
    );

    console.log("result: ");
    console.log(result);
  } finally {
    await session.close();
    await driver.close();
  }
}

run().catch((error) => {
  console.error(error);
  process.exit(1);
});
