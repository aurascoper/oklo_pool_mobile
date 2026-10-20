import Foundation
@preconcurrency import OkloCore
import SwiftUI

extension OkloSim.Field: @retroactive @unchecked Sendable {}

public struct ZoneMetadata: Sendable, Identifiable {
    public let id: Int
    public let name: String
    public let hexColor: String
    public let alpha: UInt8
    public let description: String

    public init(id: Int, name: String, hexColor: String, alpha: UInt8, description: String) {
        self.id = id
        self.name = name
        self.hexColor = hexColor
        self.alpha = alpha
        self.description = description
    }
}

public struct FieldMetadata: Sendable, Identifiable {
    public let field: OkloSim.Field
    public let displayName: String
    public let canonicalUnit: String
    public let shortUnit: String
    public let colormapName: String
    public let digits: Int
    public let isInvertedIn3D: Bool
    public let literatureAnchor: String
    public let limitationStatement: String

    public var id: UInt32 { field.rawValue }

    public init(
        field: OkloSim.Field,
        displayName: String,
        canonicalUnit: String,
        shortUnit: String,
        colormapName: String,
        digits: Int,
        isInvertedIn3D: Bool,
        literatureAnchor: String,
        limitationStatement: String
    ) {
        self.field = field
        self.displayName = displayName
        self.canonicalUnit = canonicalUnit
        self.shortUnit = shortUnit
        self.colormapName = colormapName
        self.digits = digits
        self.isInvertedIn3D = isInvertedIn3D
        self.literatureAnchor = literatureAnchor
        self.limitationStatement = limitationStatement
    }
}

public enum FieldCatalog {
    public static let allFields: [FieldMetadata] = [
        FieldMetadata(
            field: .temperature,
            displayName: "Temperature",
            canonicalUnit: "degrees C as labelled; coefficients underived",
            shortUnit: "deg C",
            colormapName: "inferno",
            digits: 2,
            isInvertedIn3D: false,
            literatureAnchor: "Cycle timing: Meshik et al. 2004, about 30 min active and 2.5 h dormant, inferred from xenon (reported observation and its interpretation); Ibekwe, Cooling, Trainer and Eaton 2020 reproduce the roughly 3 h cycle computationally (computed estimate).",
            limitationStatement: "Not a thermal-hydraulic solution. Must not be used to state temperatures at Oklo."
        ),
        FieldMetadata(
            field: .hydration,
            displayName: "Hydration",
            canonicalUnit: "pore saturation, dimensionless (0.02 to 1.0)",
            shortUnit: "saturation",
            colormapName: "blues",
            digits: 3,
            isInvertedIn3D: true,
            literatureAnchor: "Cycle timing: Meshik et al. 2004; Ibekwe et al. 2020.",
            limitationStatement: "A prescribed drying shape, not a transport solution. The 3D view highlights the driest voxels."
        ),
        FieldMetadata(
            field: .h2Concentration,
            displayName: "H2 Concentration",
            canonicalUnit: "index; labelled mmol/L with no dose basis",
            shortUnit: "mmol/L index",
            colormapName: "viridis",
            digits: 3,
            isInvertedIn3D: false,
            literatureAnchor: "G(H2) = 0.40 molecules per 100 eV for gamma radiolysis of water (Lin et al. 2005; published model parameter). H2 in Oklo fluid inclusions (Dubessy et al. 1988; reported observation).",
            limitationStatement: "A G-value alone cannot give a concentration. Never convert this field into a dose or a concentration."
        ),
        FieldMetadata(
            field: .u235Ratio,
            displayName: "235U/238U Ratio",
            canonicalUnit: "atom ratio; natural baseline 0.00725",
            shortUnit: "ratio",
            colormapName: "cividis",
            digits: 5,
            isInvertedIn3D: true,
            literatureAnchor: "Depletion at Oklo (Bodu et al. 1972; Neuilly et al. 1972; reported observation).",
            limitationStatement: "0.00725 is a ratio. It must not be swapped for 0.7204, which is an atom percentage of all uranium. Constant over the cycle."
        ),
        FieldMetadata(
            field: .polymerYield,
            displayName: "Polymer accumulation index",
            canonicalUnit: "arbitrary units (revision-0 fixture labels this in umol/L with no kinetic or thermodynamic basis)",
            shortUnit: "arbitrary units",
            colormapName: "magma",
            digits: 3,
            isInvertedIn3D: false,
            literatureAnchor: "Wet-dry condensation at a natural reactor (Ebisuzaki and Maruyama 2017; Adam et al. 2018; published hypothesis).",
            limitationStatement: "Prescribed heuristic field. The revision-0 fixture labels this quantity in umol/L, but no kinetic or thermodynamic basis for those concentration units is established. Values are not measured or predicted polymer concentrations, and its mass balance closes by construction (R0-8)."
        ),
        FieldMetadata(
            field: .compartments,
            displayName: "Compartments",
            canonicalUnit: "zone label 0 to 4 (categorical)",
            shortUnit: "zone (0 to 4)",
            colormapName: "categorical",
            digits: 0,
            isInvertedIn3D: false,
            literatureAnchor: "Prescribed geometric layout for Oklo analog system.",
            limitationStatement: "A prescribed geometry, not mapped geology. Zone 4 is applied where desiccation exceeds 0.4; no steam is modelled."
        )
    ]

    public static let zones: [ZoneMetadata] = [
        ZoneMetadata(id: 0, name: "Sandstone", hexColor: "#D8C79F", alpha: 0, description: "Host sandstone formation porous matrix"),
        ZoneMetadata(id: 1, name: "Reactor core", hexColor: "#6B2D82", alpha: 230, description: "Active uraninite natural fission zone"),
        ZoneMetadata(id: 2, name: "Clay and phosphate halo", hexColor: "#8EA86C", alpha: 40, description: "Hydrothermal alteration envelope"),
        ZoneMetadata(id: 3, name: "Fracture conduit", hexColor: "#3C7FC0", alpha: 170, description: "Vertical fracture fluid pathway"),
        ZoneMetadata(id: 4, name: "Steam chimney", hexColor: "#F0EEE8", alpha: 200, description: "Desiccation zone label where D(p) > 0.4")
    ]

    public static func metadata(for field: OkloSim.Field) -> FieldMetadata {
        guard let item = allFields.first(where: { $0.field == field }) else {
            return allFields[0]
        }
        return item
    }

    public static func zone(for id: Int) -> ZoneMetadata? {
        zones.first(where: { $0.id == id })
    }
}
